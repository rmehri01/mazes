use core::fmt;
use std::f32;

use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{
        draw_antialiased_line_segment_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
        draw_line_segment_mut,
    },
    pixelops,
    rect::Rect,
};
use petgraph::prelude::UnGraphMap;
use rand::seq::IteratorRandom;

use crate::{
    cell::{Cell, CellKind},
    distances::Distances,
    Mask,
};

pub struct Regular {
    rows: usize,
    cols: usize,
}

impl Regular {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub struct Masked(Mask);

impl Masked {
    pub fn new(mask: Mask) -> Self {
        Self(mask)
    }
}

pub struct Polar {
    rows: usize,
}

impl Polar {
    pub fn new(rows: usize) -> Self {
        Self { rows }
    }
}

pub trait GridKind
where
    Self: Sized,
{
    type Cell: CellKind;

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()>;
    // TODO: impl on grid instead?
    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell>;
}

impl GridKind for Regular {
    type Cell = Cell;

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
        let rows = self.rows;
        let cols = self.cols;

        let mut links = UnGraphMap::with_capacity(rows * cols, 0);
        for row in 0..rows {
            for col in 0..cols {
                links.add_node(Cell {
                    row: row as isize,
                    col: col as isize,
                });
            }
        }

        links
    }

    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell> {
        let north = grid.north(cell);
        let south = grid.south(cell);
        let west = grid.west(cell);
        let east = grid.east(cell);

        [north, south, west, east].into_iter().flatten()
    }
}

impl GridKind for Masked {
    type Cell = Cell;

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
        let mask = &self.0;
        let rows = mask.num_rows();
        let cols = mask.num_cols();

        let mut links = UnGraphMap::with_capacity(rows * cols, 0);
        for row in 0..rows {
            for col in 0..cols {
                if mask[row][col] {
                    links.add_node(Cell {
                        row: row as isize,
                        col: col as isize,
                    });
                }
            }
        }

        links
    }

    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell> {
        let north = grid.north(cell);
        let south = grid.south(cell);
        let west = grid.west(cell);
        let east = grid.east(cell);

        [north, south, west, east].into_iter().flatten()
    }
}

impl GridKind for Polar {
    type Cell = Cell;

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
        let rows = self.rows as f32;
        let row_height = 1.0 / rows;

        let mut links = UnGraphMap::new();
        links.add_node(Cell { row: 0, col: 0 });

        for row in 1..self.rows {
            let radius = row as f32 / rows;
            let circumference = 2.0 * f32::consts::PI * radius;

            let previous_count = row_len(&links, row as isize - 1);
            let estimated_cell_width = circumference / previous_count as f32;
            let ratio = (estimated_cell_width / row_height).round() as usize;

            let cells = previous_count * ratio;
            for col in 0..cells {
                links.add_node(Cell {
                    row: row as isize,
                    col: col as isize,
                });
            }
        }

        links
    }

    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell> {
        let clockwise = grid.clockwise(cell);
        let counter_clockwise = grid.counter_clockwise(cell);
        let inward = grid.inward(cell);

        [clockwise, counter_clockwise, inward]
            .into_iter()
            .flatten()
            .chain(grid.outward(cell))
    }
}

fn row_len(links: &UnGraphMap<Cell, ()>, r: isize) -> usize {
    links.nodes().filter(|Cell { row, .. }| *row == r).count()
}

pub struct Grid<K: GridKind> {
    kind: K,
    links: UnGraphMap<K::Cell, ()>,
    start: Option<K::Cell>,
    goal: Option<K::Cell>,
}

impl<K: GridKind> Grid<K> {
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

    pub fn dead_ends(&self) -> impl Iterator<Item = K::Cell> + '_ {
        self.cells()
            .into_iter()
            .filter(|cell| self.links(*cell).count() == 1)
    }

    pub fn distances_from(&self, cell: K::Cell) -> Distances<K> {
        let mut distances = Distances::new(cell);
        let mut frontier = vec![cell];

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();

            for cell in frontier {
                for linked in self.links(cell) {
                    if distances.get(&linked).is_none() {
                        distances.insert(linked, distances[cell] + 1);
                        new_frontier.push(linked);
                    }
                }
            }

            frontier = new_frontier;
        }

        distances
    }

    fn distances(&self) -> Option<Distances<K>> {
        match (self.start, self.goal) {
            (None, None) => None,
            (None, Some(cell)) | (Some(cell), None) => Some(self.distances_from(cell)),
            (Some(start), Some(goal)) => Some(self.distances_from(start).path_to(goal, self)),
        }
    }

    fn background_for_cell(distances: &Distances<K>, cell: K::Cell) -> Rgb<u8> {
        let distance = distances[cell];
        let (_, max) = distances.max();
        let intensity = (max - distance) as f32 / max as f32;
        let dark = (255.0 * intensity).round() as u8;
        let bright = 128 + (127.0 * intensity) as u8;
        Rgb([dark, bright, dark])
    }
}

impl Grid<Regular> {
    pub fn num_rows(&self) -> usize {
        self.kind.rows
    }
    pub fn num_cols(&self) -> usize {
        self.kind.cols
    }

    pub fn rows(&self) -> Vec<Vec<Cell>> {
        (0..self.num_rows() as isize)
            .map(|row| {
                (0..self.num_cols() as isize)
                    .map(|col| Cell { row, col })
                    .collect()
            })
            .collect()
    }
}

impl Grid<Masked> {
    pub fn num_rows(&self) -> usize {
        self.kind.0.num_rows()
    }
    pub fn num_cols(&self) -> usize {
        self.kind.0.num_cols()
    }
}

macro_rules! impl_rectangular {
    ($($T:ty),+ $(,)?) => {
        $(
            impl Grid<$T> {
                pub fn north(&self, cell: Cell) -> Option<Cell> {
                    self.get(cell.row - 1, cell.col)
                }
                pub fn south(&self, cell: Cell) -> Option<Cell> {
                    self.get(cell.row + 1, cell.col)
                }
                pub fn west(&self, cell: Cell) -> Option<Cell> {
                    self.get(cell.row, cell.col - 1)
                }
                pub fn east(&self, cell: Cell) -> Option<Cell> {
                    self.get(cell.row, cell.col + 1)
                }

                pub fn get(&self, row: isize, col: isize) -> Option<Cell> {
                    let cell = Cell { row, col };
                    self.links.contains_node(cell).then_some(cell)
                }

                pub fn save_png(&self, file_name: &str, cell_size: u32) {
                    let width = cell_size * self.num_cols() as u32;
                    let height = cell_size * self.num_rows() as u32;

                    let background = Rgb([255, 255, 255]);
                    let wall = Rgb([0, 0, 0]);

                    let mut img = RgbImage::from_pixel(width + 1, height + 1, background);

                    if let Some(distances) = self.distances() {
                        for cell in self.cells() {
                            let color = Self::background_for_cell(&distances, cell);
                            draw_filled_rect_mut(
                                &mut img,
                                Rect::at(
                                    cell.col as i32 * cell_size as i32,
                                    cell.row as i32 * cell_size as i32,
                                )
                                .of_size(cell_size, cell_size),
                                color,
                            );
                        }
                    }

                    for cell in self.cells() {
                        let x1 = cell.col as f32 * cell_size as f32;
                        let y1 = cell.row as f32 * cell_size as f32;
                        let x2 = (cell.col + 1) as f32 * cell_size as f32;
                        let y2 = (cell.row + 1) as f32 * cell_size as f32;

                        if self.north(cell).is_none() {
                            draw_line_segment_mut(&mut img, (x1, y1), (x2, y1), wall);
                        }
                        if self.west(cell).is_none() {
                            draw_line_segment_mut(&mut img, (x1, y1), (x1, y2), wall);
                        }

                        if !self
                            .east(cell)
                            .map(|east| self.are_linked(cell, east))
                            .unwrap_or(false)
                        {
                            draw_line_segment_mut(&mut img, (x2, y1), (x2, y2), wall);
                        }
                        if !self
                            .south(cell)
                            .map(|south| self.are_linked(cell, south))
                            .unwrap_or(false)
                        {
                            draw_line_segment_mut(&mut img, (x1, y2), (x2, y2), wall);
                        }
                    }

                    img.save(format!("images/{file_name}.png"))
                        .expect("image to be saved");
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
                            let cell = self.get(row, col).unwrap_or(Cell { row: -1, col: -1 });

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

impl_rectangular!(Regular, Masked);

impl Grid<Polar> {
    pub fn num_rows(&self) -> usize {
        self.kind.rows
    }

    pub fn clockwise(&self, cell: Cell) -> Option<Cell> {
        self.get(cell.row, cell.col + 1)
    }
    pub fn counter_clockwise(&self, cell: Cell) -> Option<Cell> {
        self.get(cell.row, cell.col - 1)
    }
    pub fn inward(&self, cell: Cell) -> Option<Cell> {
        if cell.row == 0 {
            return None;
        }

        let ratio = row_len(&self.links, cell.row) / row_len(&self.links, cell.row - 1);
        self.get(cell.row - 1, cell.col / ratio as isize)
    }
    pub fn outward(&self, cell: Cell) -> impl Iterator<Item = Cell> + '_ {
        self.links
            .nodes()
            .filter(move |n| self.inward(*n).is_some_and(|c| c == cell))
    }

    pub fn get(&self, row: isize, col: isize) -> Option<Cell> {
        let cell = Cell {
            row,
            col: col % row_len(&self.links, row) as isize,
        };
        self.links.contains_node(cell).then_some(cell)
    }

    pub fn save_png(&self, file_name: &str, cell_size: u32) {
        let img_size = 2 * self.num_rows() as u32 * cell_size;

        let background = Rgb([255, 255, 255]);
        let wall = Rgb([0, 0, 0]);

        let mut img = RgbImage::from_pixel(img_size + 1, img_size + 1, background);
        let center = img_size as i32 / 2;

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
            let cx = center + (inner_radius * theta_cw.cos()) as i32;
            let cy = center + (inner_radius * theta_cw.sin()) as i32;
            let dx = center + (outer_radius * theta_cw.cos()) as i32;
            let dy = center + (outer_radius * theta_cw.sin()) as i32;

            if !self
                .inward(cell)
                .map(|north| self.are_linked(cell, north))
                .unwrap_or(false)
            {
                draw_antialiased_line_segment_mut(
                    &mut img,
                    (ax, ay),
                    (cx, cy),
                    wall,
                    pixelops::interpolate,
                );
            }
            if !self
                .clockwise(cell)
                .map(|east| self.are_linked(cell, east))
                .unwrap_or(false)
            {
                draw_antialiased_line_segment_mut(
                    &mut img,
                    (cx, cy),
                    (dx, dy),
                    wall,
                    pixelops::interpolate,
                );
            }
        }

        draw_hollow_circle_mut(
            &mut img,
            (center, center),
            self.num_rows() as i32 * cell_size as i32,
            wall,
        );

        img.save(format!("images/{file_name}.png"))
            .expect("image to be saved");
    }
}
