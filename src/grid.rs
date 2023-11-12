use core::fmt;
use std::{cmp::Ordering, collections::BinaryHeap, f32};

use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::{
    drawing::{
        draw_antialiased_line_segment_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
        draw_polygon_mut,
    },
    pixelops,
    point::Point,
    rect::Rect,
};
use petgraph::prelude::UnGraphMap;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use rustc_hash::FxHashMap;

use crate::{
    cell::{CellKind, HexCell, PolarCell, RegularCell, TriangleCell, WeightedCell},
    distances::Distances,
    kind::{Hex, Kind, Masked, Polar, Regular, Triangle, Weighted},
};

const BACKGROUND: Rgb<u8> = Rgb([255, 255, 255]);
const WALL: Rgb<u8> = Rgb([0, 0, 0]);

pub struct Grid<K: Kind> {
    kind: K,
    links: UnGraphMap<K::Cell, ()>,
    start: Option<K::Cell>,
    goal: Option<K::Cell>,
}

impl<K: Kind> Grid<K> {
    pub fn new(kind: K, start: Option<K::Cell>, goal: Option<K::Cell>) -> Self {
        let links = kind.prepare_grid();

        Self {
            kind,
            links,
            start,
            goal,
        }
    }

    pub fn set_start(&mut self, start: K::Cell) {
        self.start = Some(start);
    }
    pub fn set_goal(&mut self, start: K::Cell) {
        self.goal = Some(start);
    }

    pub fn cells(&self) -> Vec<K::Cell> {
        self.links.nodes().collect()
    }

    pub fn link(&mut self, cell: K::Cell, other: K::Cell) {
        self.links.add_edge(cell, other, ());
    }
    pub fn unlink(&mut self, cell: K::Cell, other: K::Cell) {
        self.links.remove_edge(cell, other);
    }

    pub fn links(&self, cell: K::Cell) -> impl Iterator<Item = K::Cell> + '_ {
        self.links.neighbors(cell)
    }
    pub fn are_linked(&self, cell: K::Cell, other: K::Cell) -> bool {
        self.links.contains_edge(cell, other)
    }

    pub fn neighbours(&self, cell: K::Cell) -> impl Iterator<Item = K::Cell> + '_ {
        K::neighbours(self, cell)
    }

    pub fn get_random_cell(&self) -> K::Cell {
        self.links
            .nodes()
            .choose(&mut rand::thread_rng())
            .expect("at least one cell in the grid")
    }

    pub fn size(&self) -> usize {
        self.links.node_count()
    }

    pub fn dead_ends(&self) -> Vec<K::Cell> {
        self.cells()
            .into_iter()
            .filter(|cell| self.links(*cell).count() == 1)
            .collect()
    }

    pub fn braid(mut self, p: f32) -> Self {
        let mut dead_ends = self.dead_ends();
        dead_ends.shuffle(&mut rand::thread_rng());

        for cell in dead_ends {
            if self.links(cell).count() != 1 || rand::thread_rng().gen_range(0.0..=1.0) > p {
                continue;
            }

            let neighbours = self
                .neighbours(cell)
                .filter(|n| !self.are_linked(cell, *n))
                .collect::<Vec<_>>();
            let dead_end_neighbours = neighbours
                .iter()
                .filter(|n| self.links(**n).count() == 1)
                .copied();

            let neighbour = dead_end_neighbours
                .choose(&mut rand::thread_rng())
                .unwrap_or_else(|| {
                    *neighbours
                        .choose(&mut rand::thread_rng())
                        .expect("neighbours should be non-empty")
                });
            self.link(cell, neighbour);
        }

        self
    }

    pub fn distances_from(&self, cell: K::Cell) -> Distances<K> {
        // Explicitly implement Ord to get a min-heap.
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State<T: CellKind> {
            cost: usize,
            cell: T,
        }

        impl<T: CellKind> Ord for State<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                other
                    .cost
                    .cmp(&self.cost)
                    .then_with(|| self.cell.cmp(&other.cell))
            }
        }

        impl<T: CellKind> PartialOrd for State<T> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut weights = Distances::new(cell);
        let mut pending = BinaryHeap::from_iter([State { cost: 0, cell }]);

        while let Some(State { cell, cost }) = pending.pop() {
            for neighbour in self.links(cell) {
                let total_weight = cost + neighbour.weight();

                if weights.get(&neighbour).is_none() || total_weight < weights[neighbour] {
                    pending.push(State {
                        cost: total_weight,
                        cell: neighbour,
                    });
                    weights.insert(neighbour, total_weight);
                }
            }
        }

        weights
    }

    pub fn distances(&self) -> Option<Distances<K>> {
        match (self.start, self.goal) {
            (None, None) => None,
            (None, Some(cell)) | (Some(cell), None) => Some(self.distances_from(cell)),
            (Some(start), Some(goal)) => Some(self.distances_from(start).path_to(goal, self)),
        }
    }

    fn background_for_cell(distances: &Distances<K>, cell: K::Cell) -> Option<Rgb<u8>> {
        if cell.weight() > 1 {
            Some(Rgb([255, 0, 0]))
        } else {
            let distance = distances.get(&cell)?;
            let (_, max) = distances.max();
            let intensity = (max - distance) as f32 / max as f32;
            let dark = (255.0 * intensity).round() as u8;
            let bright = 128 + (127.0 * intensity) as u8;
            Some(Rgb([dark, bright, dark]))
        }
    }

    pub fn num_rows(&self) -> usize {
        self.kind.num_rows()
    }
}

impl Grid<Regular> {
    pub fn num_cols(&self) -> usize {
        self.kind.cols
    }

    pub fn rows(&self) -> Vec<Vec<RegularCell>> {
        (0..self.num_rows() as isize)
            .map(|row| {
                (0..self.num_cols() as isize)
                    .map(|col| RegularCell { row, col })
                    .collect()
            })
            .collect()
    }
}

impl Grid<Masked> {
    pub fn num_cols(&self) -> usize {
        self.kind.0.num_cols()
    }
}

macro_rules! impl_rectangular {
    ($($T:ty),+ $(,)?) => {
        $(
            impl Grid<$T> {
                pub fn north(&self, cell: <$T as Kind>::Cell) -> Option<<$T as Kind>::Cell> {
                    self.get(cell.row - 1, cell.col)
                }
                pub fn south(&self, cell: <$T as Kind>::Cell) -> Option<<$T as Kind>::Cell> {
                    self.get(cell.row + 1, cell.col)
                }
                pub fn west(&self, cell: <$T as Kind>::Cell) -> Option<<$T as Kind>::Cell> {
                    self.get(cell.row, cell.col - 1)
                }
                pub fn east(&self, cell: <$T as Kind>::Cell) -> Option<<$T as Kind>::Cell> {
                    self.get(cell.row, cell.col + 1)
                }

                pub fn get(&self, row: isize, col: isize) -> Option<<$T as Kind>::Cell> {
                    self.links.nodes().find(|n| n.row == row && n.col == col)
                }

                pub fn save_png(&self, file_name: &str, cell_size: u32, inset: f32) {
                    let img_width = cell_size * self.num_cols() as u32;
                    let img_height = cell_size * self.num_rows() as u32;
                    let inset = (cell_size as f32 * inset) as u32;

                    let mut img = RgbImage::from_pixel(img_width + 1, img_height + 1, BACKGROUND);

                    let modes = [
                        self.distances().map(SavePngMode::Background),
                        Some(SavePngMode::Walls),
                    ]
                    .into_iter()
                    .flatten();
                    for mode in modes {
                        for cell in self.cells() {
                            let x = cell.col as i32 * cell_size as i32;
                            let y = cell.row as i32 * cell_size as i32;

                            if inset > 0 {
                                self.render_cell_with_inset(&mut img, cell, &mode, cell_size, (x, y), inset);
                            } else {
                                self.render_cell_without_inset(&mut img, cell, &mode, cell_size, (x, y));
                            }
                        }
                    }

                    img.save(format!("images/{file_name}.png"))
                        .expect("image to be saved");
                }

                fn render_cell_with_inset(
                    &self,
                    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
                    cell: <$T as Kind>::Cell,
                    mode: &SavePngMode<$T>,
                    cell_size: u32,
                    xy: (i32, i32),
                    inset: u32,
                ) {
                    let [x1, x2, x3, x4, y1, y2, y3, y4] =
                        Self::cell_coordinates_with_inset(xy, cell_size, inset);

                    match mode {
                        SavePngMode::Background(distances) => {
                            if let Some(color) = Self::background_for_cell(distances, cell) {
                                let inner_size = cell_size - 2 * inset;

                                draw_filled_rect_mut(
                                    img,
                                    Rect::at(x2, y2).of_size(inner_size, inner_size),
                                    color,
                                );

                                if self
                                    .north(cell)
                                    .map(|north| self.are_linked(cell, north))
                                    .unwrap_or(false)
                                {
                                    draw_filled_rect_mut(
                                        img,
                                        Rect::at(x2, y1).of_size(inner_size, inset),
                                        color,
                                    );
                                }

                                if self
                                    .south(cell)
                                    .map(|south| self.are_linked(cell, south))
                                    .unwrap_or(false)
                                {
                                    draw_filled_rect_mut(
                                        img,
                                        Rect::at(x2, y3).of_size(inner_size, inset),
                                        color,
                                    );
                                }

                                if self
                                    .west(cell)
                                    .map(|west| self.are_linked(cell, west))
                                    .unwrap_or(false)
                                {
                                    draw_filled_rect_mut(
                                        img,
                                        Rect::at(x1, y2).of_size(inset, inner_size),
                                        color,
                                    );
                                }

                                if self
                                    .east(cell)
                                    .map(|east| self.are_linked(cell, east))
                                    .unwrap_or(false)
                                {
                                    draw_filled_rect_mut(
                                        img,
                                        Rect::at(x3, y2).of_size(inset, inner_size),
                                        color,
                                    );
                                }
                            }
                        }
                        SavePngMode::Walls => {
                            if self
                                .north(cell)
                                .map(|north| self.are_linked(cell, north))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y1),
                                    (x2, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x3, y1),
                                    (x3, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            } else {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y2),
                                    (x3, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }

                            if self
                                .south(cell)
                                .map(|south| self.are_linked(cell, south))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y3),
                                    (x2, y4),
                                    WALL,
                                    pixelops::interpolate,
                                );
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x3, y3),
                                    (x3, y4),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            } else {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y3),
                                    (x3, y3),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }

                            if self
                                .west(cell)
                                .map(|west| self.are_linked(cell, west))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x1, y2),
                                    (x2, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x1, y3),
                                    (x2, y3),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            } else {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y2),
                                    (x2, y3),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }

                            if self
                                .east(cell)
                                .map(|east| self.are_linked(cell, east))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x3, y2),
                                    (x4, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x3, y3),
                                    (x4, y3),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            } else {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x3, y2),
                                    (x3, y3),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }
                        }
                    }
                }

                fn cell_coordinates_with_inset(xy: (i32, i32), cell_size: u32, inset: u32) -> [i32; 8] {
                    let cell_size = cell_size as i32;
                    let inset = inset as i32;

                    let (x1, y1) = xy;
                    let x4 = x1 + cell_size;
                    let x2 = x1 + inset;
                    let x3 = x4 - inset;

                    let y4 = y1 + cell_size;
                    let y2 = y1 + inset;
                    let y3 = y4 - inset;

                    [x1, x2, x3, x4, y1, y2, y3, y4]
                }

                fn render_cell_without_inset(
                    &self,
                    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
                    cell: <$T as Kind>::Cell,
                    mode: &SavePngMode<$T>,
                    cell_size: u32,
                    xy: (i32, i32),
                ) {
                    let (x1, y1) = xy;
                    let x2 = x1 + cell_size as i32;
                    let y2 = y1 + cell_size as i32;

                    match mode {
                        SavePngMode::Background(distances) => {
                            if let Some(color) = Self::background_for_cell(distances, cell) {
                                draw_filled_rect_mut(
                                    img,
                                    Rect::at(x1, y1).of_size(cell_size, cell_size),
                                    color,
                                );
                            }
                        }
                        SavePngMode::Walls => {
                            if self.north(cell).is_none() {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x1, y1),
                                    (x2, y1),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }
                            if self.west(cell).is_none() {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x1, y1),
                                    (x1, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }

                            if !self
                                .east(cell)
                                .map(|east| self.are_linked(cell, east))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x2, y1),
                                    (x2, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }
                            if !self
                                .south(cell)
                                .map(|south| self.are_linked(cell, south))
                                .unwrap_or(false)
                            {
                                draw_antialiased_line_segment_mut(
                                    img,
                                    (x1, y2),
                                    (x2, y2),
                                    WALL,
                                    pixelops::interpolate,
                                );
                            }
                        }
                    }
                }
            }

            impl fmt::Display for Grid<$T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    let is_space_between_empty = |cell, other| match (cell, other) {
                        // both outside the grid, so space should be empty
                        (None, None) => true,
                        // one is outside the grid, so space should not be empty since it's an outer wall
                        (None, Some(_)) | (Some(_), None) => false,
                        // the space should be empty if the two cells are linked
                        (Some(cell), Some(other)) => self.are_linked(cell, other),
                    };

                    let connector_at = |row, col| {
                        let empty_north =
                            is_space_between_empty(self.get(row - 1, col - 1), self.get(row - 1, col));
                        let empty_south =
                            is_space_between_empty(self.get(row, col - 1), self.get(row, col));
                        let empty_west =
                            is_space_between_empty(self.get(row - 1, col - 1), self.get(row, col - 1));
                        let empty_east =
                            is_space_between_empty(self.get(row - 1, col), self.get(row, col));

                        match (empty_north, empty_south, empty_west, empty_east) {
                            (false, true, true, true) => '╵',
                            (true, false, true, true) => '╷',
                            (true, true, false, true) => '╴',
                            (true, true, true, false) => '╶',
                            (true, false, false, false) => '┬',
                            (false, true, false, false) => '┴',
                            (false, false, true, false) => '├',
                            (false, false, false, true) => '┤',
                            (true, true, false, false) => '─',
                            (true, false, true, false) => '┌',
                            (true, false, false, true) => '┐',
                            (false, true, true, false) => '└',
                            (false, true, false, true) => '┘',
                            (false, false, true, true) => '│',
                            (false, false, false, false) => '┼',
                            (true, true, true, true) => ' ',
                        }
                    };

                    let distances = self.distances();

                    writeln!(
                        f,
                        "{}",
                        (1..=self.num_cols() as isize).fold(
                            connector_at(0, 0).to_string(),
                            |mut acc, col| {
                                acc.push_str("───");
                                acc.push(connector_at(0, col));
                                acc
                            }
                        )
                    )?;

                    for row in 0..self.num_rows() as isize {
                        let mut top = "│".to_string();
                        let mut bot = connector_at(row + 1, 0).to_string();

                        for col in 0..self.num_cols() as isize {
                            let cell = self.get(row, col).unwrap_or(<$T as Kind>::Cell::new(-1, -1));

                            let formatted_dist = distances
                                .as_ref()
                                .and_then(|distances| distances.get(&cell))
                                .map(|dist| format!("{:>3}", dist));
                            top.push_str(formatted_dist.as_deref().unwrap_or("   "));
                            let east_boundary = if self
                                .east(cell)
                                .map(|east| self.are_linked(cell, east))
                                .unwrap_or(false)
                            {
                                ' '
                            } else {
                                '│'
                            };
                            top.push(east_boundary);

                            let south_boundary = if self
                                .south(cell)
                                .map(|south| self.are_linked(cell, south))
                                .unwrap_or(false)
                            {
                                "   "
                            } else {
                                "───"
                            };
                            bot.push_str(south_boundary);
                            bot.push(connector_at(row + 1, col + 1));
                        }

                        writeln!(f, "{top}")?;
                        writeln!(f, "{bot}")?;
                    }

                    Ok(())
                }
            }
        )+
    };
}

impl_rectangular!(Regular, Masked, Weighted);

impl Grid<Polar> {
    pub fn clockwise(&self, cell: PolarCell) -> Option<PolarCell> {
        self.get(cell.row, cell.col + 1)
    }
    pub fn counter_clockwise(&self, cell: PolarCell) -> Option<PolarCell> {
        self.get(cell.row, cell.col - 1)
    }
    pub fn inward(&self, cell: PolarCell) -> Option<PolarCell> {
        if cell.row == 0 {
            return None;
        }

        let ratio = row_len(&self.links, cell.row) / row_len(&self.links, cell.row - 1);
        self.get(cell.row - 1, cell.col / ratio as isize)
    }
    pub fn outward(&self, cell: PolarCell) -> impl Iterator<Item = PolarCell> {
        let ratio = (row_len(&self.links, cell.row + 1) / row_len(&self.links, cell.row)) as isize;

        (cell.col * ratio..cell.col * ratio + ratio).map(move |col| PolarCell {
            row: cell.row + 1,
            col,
        })
    }

    pub fn get(&self, row: isize, col: isize) -> Option<PolarCell> {
        let cell = PolarCell {
            row,
            col: col % row_len(&self.links, row) as isize,
        };
        self.links.contains_node(cell).then_some(cell)
    }

    pub fn save_png(&self, file_name: &str, cell_size: u32) {
        struct CellCoords {
            a: (i32, i32),
            b: (i32, i32),
            c: (i32, i32),
            d: (i32, i32),
        }

        let img_size = 2 * self.num_rows() as u32 * cell_size;

        let mut img = RgbImage::from_pixel(img_size + 1, img_size + 1, BACKGROUND);
        let center = img_size as i32 / 2;

        let mut coord_map = FxHashMap::default();
        for cell in self.cells() {
            if cell.row == 0 {
                continue;
            }

            let theta = 2.0 * f32::consts::PI / row_len(&self.links, cell.row) as f32;
            let inner_radius = cell.row as f32 * cell_size as f32;
            let outer_radius = (cell.row + 1) as f32 * cell_size as f32;
            let theta_ccw = cell.col as f32 * theta;
            let theta_cw = (cell.col + 1) as f32 * theta;

            let ax = center + (inner_radius * theta_ccw.cos()) as i32;
            let ay = center + (inner_radius * theta_ccw.sin()) as i32;
            let bx = center + (outer_radius * theta_ccw.cos()) as i32;
            let by = center + (outer_radius * theta_ccw.sin()) as i32;
            let cx = center + (inner_radius * theta_cw.cos()) as i32;
            let cy = center + (inner_radius * theta_cw.sin()) as i32;
            let dx = center + (outer_radius * theta_cw.cos()) as i32;
            let dy = center + (outer_radius * theta_cw.sin()) as i32;

            coord_map.insert(
                cell,
                CellCoords {
                    a: (ax, ay),
                    b: (bx, by),
                    c: (cx, cy),
                    d: (dx, dy),
                },
            );
        }

        if let Some(distances) = self.distances() {
            for cell in self.cells() {
                if let Some(color) = Self::background_for_cell(&distances, cell) {
                    if cell.row == 0 {
                        let poly = self
                            .outward(cell)
                            .flat_map(|c| {
                                let CellCoords {
                                    a: (out_ax, out_ay),
                                    c: (out_cx, out_cy),
                                    ..
                                } = coord_map[&c];

                                [Point::new(out_ax, out_ay), Point::new(out_cx, out_cy)]
                            })
                            .skip(1) // polygon needs to be open
                            .collect::<Vec<_>>();
                        draw_polygon_mut(&mut img, &poly, color);
                    } else {
                        let CellCoords {
                            a: (ax, ay),
                            b: (bx, by),
                            c: (cx, cy),
                            d: (dx, dy),
                        } = coord_map[&cell];

                        let poly = match self.outward(cell).next() {
                            Some(out) => {
                                let (out_cx, out_cy) = coord_map[&out].c;

                                vec![
                                    Point::new(cx, cy),
                                    Point::new(dx, dy),
                                    Point::new(out_cx, out_cy),
                                    Point::new(bx, by),
                                    Point::new(ax, ay),
                                ]
                            }
                            None => vec![
                                Point::new(cx, cy),
                                Point::new(dx, dy),
                                Point::new(bx, by),
                                Point::new(ax, ay),
                            ],
                        };
                        draw_polygon_mut(&mut img, &poly, color);
                    }
                }
            }
        }

        for cell in self.cells() {
            if cell.row == 0 {
                continue;
            }

            let CellCoords {
                a: (ax, ay),
                b: _,
                c: (cx, cy),
                d: (dx, dy),
            } = coord_map[&cell];

            if !self
                .inward(cell)
                .map(|inward| self.are_linked(cell, inward))
                .unwrap_or(false)
            {
                draw_antialiased_line_segment_mut(
                    &mut img,
                    (ax, ay),
                    (cx, cy),
                    WALL,
                    pixelops::interpolate,
                );
            }
            if !self
                .clockwise(cell)
                .map(|clockwise| self.are_linked(cell, clockwise))
                .unwrap_or(false)
            {
                draw_antialiased_line_segment_mut(
                    &mut img,
                    (cx, cy),
                    (dx, dy),
                    WALL,
                    pixelops::interpolate,
                );
            }
        }

        draw_hollow_circle_mut(
            &mut img,
            (center, center),
            self.num_rows() as i32 * cell_size as i32,
            WALL,
        );

        img.save(format!("images/{file_name}.png"))
            .expect("image to be saved");
    }
}

impl Grid<Hex> {
    pub fn num_cols(&self) -> usize {
        self.kind.cols
    }

    pub fn north_west(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.north_diagonal_row(), cell.col - 1)
    }
    pub fn north(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.row - 1, cell.col)
    }
    pub fn north_east(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.north_diagonal_row(), cell.col + 1)
    }
    pub fn south_west(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.south_diagonal_row(), cell.col - 1)
    }
    pub fn south(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.row + 1, cell.col)
    }
    pub fn south_east(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.south_diagonal_row(), cell.col + 1)
    }

    pub fn get(&self, row: isize, col: isize) -> Option<HexCell> {
        let cell = HexCell { row, col };
        self.links.contains_node(cell).then_some(cell)
    }
    pub fn get_next_in_row(&self, cell: HexCell) -> Option<HexCell> {
        self.get(cell.row, cell.col + 1)
    }

    pub fn rows(&self) -> Vec<Vec<HexCell>> {
        (0..self.num_rows() as isize)
            .map(|row| {
                (0..self.num_cols() as isize)
                    .map(|col| HexCell { row, col })
                    .collect()
            })
            .collect()
    }

    pub fn save_png(&self, file_name: &str, cell_size: u32) {
        let cell_size = cell_size as f32;

        let a_size = cell_size / 2.0;
        let b_size = cell_size * 3.0_f32.sqrt() / 2.0;
        let height = b_size * 2.0;

        let img_width = (3.0 * a_size * self.num_cols() as f32 + a_size + 0.5) as u32;
        let img_height = (height * self.num_rows() as f32 + b_size + 0.5) as u32;

        let mut img = RgbImage::from_pixel(img_width + 1, img_height + 1, BACKGROUND);

        let modes = [
            self.distances().map(SavePngMode::Background),
            Some(SavePngMode::Walls),
        ]
        .into_iter()
        .flatten();
        for mode in modes {
            for cell in self.cells() {
                let cx = cell_size + 3.0 * cell.col as f32 * a_size;
                let mut cy = b_size + cell.row as f32 * height;
                if cell.col % 2 == 1 {
                    cy += b_size;
                }

                // f/n = far/near
                // n/s/e/w = north/south/east/west
                let x_fw = (cx - cell_size) as i32;
                let x_nw = (cx - a_size) as i32;
                let x_ne = (cx + a_size) as i32;
                let x_fe = (cx + cell_size) as i32;

                // m = middle
                let y_n = (cy - b_size) as i32;
                let y_m = cy as i32;
                let y_s = (cy + b_size) as i32;

                match mode {
                    SavePngMode::Background(ref distances) => {
                        if let Some(color) = Self::background_for_cell(distances, cell) {
                            draw_polygon_mut(
                                &mut img,
                                &[
                                    Point::new(x_fw, y_m),
                                    Point::new(x_nw, y_n),
                                    Point::new(x_ne, y_n),
                                    Point::new(x_fe, y_m),
                                    Point::new(x_ne, y_s),
                                    Point::new(x_nw, y_s),
                                ],
                                color,
                            );
                        }
                    }
                    SavePngMode::Walls => {
                        if self.south_west(cell).is_none() {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_fw, y_m),
                                (x_nw, y_s),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                        if self.north_west(cell).is_none() {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_fw, y_m),
                                (x_nw, y_n),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                        if self.north(cell).is_none() {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_nw, y_n),
                                (x_ne, y_n),
                                WALL,
                                pixelops::interpolate,
                            );
                        }

                        if !self
                            .north_east(cell)
                            .map(|north_east| self.are_linked(cell, north_east))
                            .unwrap_or(false)
                        {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_ne, y_n),
                                (x_fe, y_m),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                        if !self
                            .south_east(cell)
                            .map(|south_east| self.are_linked(cell, south_east))
                            .unwrap_or(false)
                        {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_fe, y_m),
                                (x_ne, y_s),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                        if !self
                            .south(cell)
                            .map(|south| self.are_linked(cell, south))
                            .unwrap_or(false)
                        {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (x_ne, y_s),
                                (x_nw, y_s),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                    }
                }
            }
        }

        img.save(format!("images/{file_name}.png"))
            .expect("image to be saved");
    }
}

impl Grid<Triangle> {
    pub fn num_cols(&self) -> usize {
        self.kind.cols
    }

    pub fn north(&self, cell: TriangleCell) -> Option<TriangleCell> {
        if cell.is_upright() {
            return None;
        }

        self.get(cell.row - 1, cell.col)
    }
    pub fn south(&self, cell: TriangleCell) -> Option<TriangleCell> {
        if !cell.is_upright() {
            return None;
        }

        self.get(cell.row + 1, cell.col)
    }
    pub fn west(&self, cell: TriangleCell) -> Option<TriangleCell> {
        self.get(cell.row, cell.col - 1)
    }
    pub fn east(&self, cell: TriangleCell) -> Option<TriangleCell> {
        self.get(cell.row, cell.col + 1)
    }

    pub fn get(&self, row: isize, col: isize) -> Option<TriangleCell> {
        let cell = TriangleCell { row, col };
        self.links.contains_node(cell).then_some(cell)
    }

    pub fn save_png(&self, file_name: &str, cell_size: u32) {
        let cell_size = cell_size as f32;

        let half_width = cell_size / 2.0;
        let height = cell_size * 3.0_f32.sqrt() / 2.0;
        let half_height = height / 2.0;

        let img_width = (cell_size * (self.num_cols() + 1) as f32 / 2.0) as u32;
        let img_height = (height * self.num_rows() as f32) as u32;

        let mut img = RgbImage::from_pixel(img_width + 1, img_height + 1, BACKGROUND);

        let modes = [
            self.distances().map(SavePngMode::Background),
            Some(SavePngMode::Walls),
        ]
        .into_iter()
        .flatten();
        for mode in modes {
            for cell in self.cells() {
                let cx = half_width + cell.col as f32 * half_width;
                let cy = half_height + cell.row as f32 * height;

                let west_x = (cx - half_width) as i32;
                let mid_x = cx as i32;
                let east_x = (cx + half_width) as i32;

                let (base_y, apex_y) = if cell.is_upright() {
                    ((cy + half_height) as i32, (cy - half_height) as i32)
                } else {
                    ((cy - half_height) as i32, (cy + half_height) as i32)
                };

                match mode {
                    SavePngMode::Background(ref distances) => {
                        if let Some(color) = Self::background_for_cell(distances, cell) {
                            draw_polygon_mut(
                                &mut img,
                                &[
                                    Point::new(west_x, base_y),
                                    Point::new(mid_x, apex_y),
                                    Point::new(east_x, base_y),
                                ],
                                color,
                            );
                        }
                    }
                    SavePngMode::Walls => {
                        if self.west(cell).is_none() {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (west_x, base_y),
                                (mid_x, apex_y),
                                WALL,
                                pixelops::interpolate,
                            );
                        }

                        if !self
                            .east(cell)
                            .map(|east| self.are_linked(cell, east))
                            .unwrap_or(false)
                        {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (east_x, base_y),
                                (mid_x, apex_y),
                                WALL,
                                pixelops::interpolate,
                            );
                        }

                        let no_south = cell.is_upright() && self.south(cell).is_none();
                        let not_linked = !cell.is_upright()
                            && !self
                                .north(cell)
                                .map(|north| self.are_linked(cell, north))
                                .unwrap_or(false);
                        if no_south || not_linked {
                            draw_antialiased_line_segment_mut(
                                &mut img,
                                (east_x, base_y),
                                (west_x, base_y),
                                WALL,
                                pixelops::interpolate,
                            );
                        }
                    }
                }
            }
        }

        img.save(format!("images/{file_name}.png"))
            .expect("image to be saved");
    }
}

impl Grid<Weighted> {
    pub fn num_cols(&self) -> usize {
        self.kind.cols
    }

    pub fn set_weight(&mut self, row: isize, col: isize, weight: usize) {
        let cell = self
            .links
            .nodes()
            .find(|n| n.row == row && n.col == col)
            .expect("cell to be found");
        let connected = self
            .links
            .edges(cell)
            .map(|(_, other, _)| other)
            .collect::<Vec<_>>();
        self.links.remove_node(cell);

        let cell = self.links.add_node(WeightedCell { row, col, weight });
        for other in connected {
            self.links.add_edge(cell, other, ());
        }
    }
}

enum SavePngMode<K: Kind> {
    Background(Distances<K>),
    Walls,
}

pub(crate) fn row_len(links: &UnGraphMap<impl CellKind, ()>, r: isize) -> usize {
    links.nodes().filter(|c| c.row() == r).count()
}
