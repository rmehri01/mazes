use core::fmt;

use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment_mut},
    rect::Rect,
};
use petgraph::prelude::UnGraphMap;
use rand::seq::IteratorRandom;

use crate::{cell::Cell, distances::Distances, Mask};

pub struct Grid {
    rows: usize,
    cols: usize,
    links: UnGraphMap<Cell, ()>,
    start: Option<Cell>,
    goal: Option<Cell>,
}

impl Grid {
    pub fn new(mask: &Mask, start: Option<Cell>, goal: Option<Cell>) -> Self {
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

        Self {
            rows,
            cols,
            links,
            start,
            goal,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows
    }
    pub fn num_cols(&self) -> usize {
        self.cols
    }

    pub fn set_start(&mut self, start: Cell) {
        self.start = Some(start);
    }
    pub fn set_goal(&mut self, start: Cell) {
        self.goal = Some(start);
    }

    pub fn cells(&self) -> Vec<Cell> {
        self.links.nodes().collect()
    }

    pub fn rows(&self) -> Vec<Vec<Cell>> {
        (0..self.rows as isize)
            .map(|r| {
                self.links
                    .nodes()
                    .filter(|Cell { row, .. }| *row == r)
                    .collect()
            })
            .collect()
    }

    // TODO: move these to cell?
    pub fn link(&mut self, cell: Cell, other: Cell) {
        self.links.add_edge(cell, other, ());
    }
    pub fn unlink(&mut self, cell: Cell, other: Cell) {
        self.links.remove_edge(cell, other);
    }

    pub fn links(&self, cell: Cell) -> impl Iterator<Item = Cell> + '_ {
        self.links.neighbors(cell)
    }
    pub fn are_linked(&self, cell: Cell, other: Cell) -> bool {
        self.links.contains_edge(cell, other)
    }

    pub fn neighbours(&self, cell: Cell) -> impl Iterator<Item = Cell> {
        let north = self.north(cell);
        let south = self.south(cell);
        let west = self.west(cell);
        let east = self.east(cell);

        [north, south, west, east].into_iter().flatten()
    }

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
    pub fn get_random_cell(&self) -> Cell {
        self.links
            .nodes()
            .choose(&mut rand::thread_rng())
            .expect("at least one cell in the grid")
    }

    pub fn size(&self) -> usize {
        self.links.node_count()
    }

    pub fn dead_ends(&self) -> impl Iterator<Item = Cell> + '_ {
        self.cells()
            .into_iter()
            .filter(|cell| self.links(*cell).count() == 1)
    }

    pub fn distances_from(&self, cell: Cell) -> Distances {
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

    pub fn save_png(&self, file_name: &str, cell_size: u32) {
        fn background_for_cell(distances: &Distances, cell: Cell) -> Rgb<u8> {
            let distance = distances[cell];
            let (_, max) = distances.max();
            let intensity = (max - distance) as f32 / max as f32;
            let dark = (255.0 * intensity).round() as u8;
            let bright = 128 + (127.0 * intensity) as u8;
            Rgb([dark, bright, dark])
        }

        let width = cell_size * self.cols as u32;
        let height = cell_size * self.rows as u32;

        let background = Rgb([255, 255, 255]);
        let wall = Rgb([0, 0, 0]);

        let mut img = RgbImage::from_pixel(width + 1, height + 1, background);

        if let Some(distances) = self.distances() {
            for cell in self.cells() {
                let color = background_for_cell(&distances, cell);
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

    fn distances(&self) -> Option<Distances> {
        match (self.start, self.goal) {
            (None, None) => None,
            (None, Some(cell)) | (Some(cell), None) => Some(self.distances_from(cell)),
            (Some(start), Some(goal)) => Some(self.distances_from(start).path_to(goal, self)),
        }
    }
}

impl fmt::Display for Grid {
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
            let empty_south = is_space_between_empty(self.get(row, col - 1), self.get(row, col));
            let empty_west =
                is_space_between_empty(self.get(row - 1, col - 1), self.get(row, col - 1));
            let empty_east = is_space_between_empty(self.get(row - 1, col), self.get(row, col));

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
            (1..=self.cols as isize).fold(connector_at(0, 0).to_string(), |mut acc, col| {
                acc.push_str("───");
                acc.push(connector_at(0, col));
                acc
            })
        )?;

        for row in 0..self.rows as isize {
            let mut top = "│".to_string();
            let mut bot = connector_at(row + 1, 0).to_string();

            for col in 0..self.cols as isize {
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
