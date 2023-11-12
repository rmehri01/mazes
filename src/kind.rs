use std::f32;

use petgraph::prelude::UnGraphMap;

use crate::{
    cell::{Cell, CellKind},
    grid::{row_len, Grid},
    mask::Mask,
};

pub struct Regular {
    pub rows: usize,
    pub cols: usize,
}

impl Regular {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub struct Masked(pub Mask);

impl Masked {
    pub fn new(mask: Mask) -> Self {
        Self(mask)
    }
}

pub struct Polar {
    pub rows: usize,
}

impl Polar {
    pub fn new(rows: usize) -> Self {
        Self { rows }
    }
}

pub struct Hex {
    pub rows: usize,
    pub cols: usize,
}

impl Hex {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub trait Kind
where
    Self: Sized,
{
    type Cell: CellKind;

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()>;
    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell>;
}

impl Kind for Regular {
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

impl Kind for Masked {
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

impl Kind for Polar {
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

impl Kind for Hex {
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
        let north_west = grid.north_west(cell);
        let north = grid.north(cell);
        let north_east = grid.north_east(cell);
        let south_west = grid.south_west(cell);
        let south = grid.south(cell);
        let south_east = grid.south_east(cell);

        [north_west, north, north_east, south_west, south, south_east]
            .into_iter()
            .flatten()
    }
}
