use std::f32;

use petgraph::prelude::UnGraphMap;

use crate::{
    cell::{CellKind, HexCell, PolarCell, RegularCell, TriangleCell, WeaveCell, WeightedCell},
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

pub struct Triangle {
    pub rows: usize,
    pub cols: usize,
}

impl Triangle {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub struct Weighted {
    pub rows: usize,
    pub cols: usize,
}

impl Weighted {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub struct Weave {
    pub rows: usize,
    pub cols: usize,
}

impl Weave {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

pub trait Kind
where
    Self: Sized,
{
    type Cell: CellKind;

    fn num_rows(&self) -> usize;
    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()>;

    fn link(grid: &mut Grid<Self>, cell: Self::Cell, other: Self::Cell) {
        grid.connect(cell, other);
    }
    fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell>;
}

macro_rules! default_prepare_grid {
    () => {
        fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
            let rows = self.rows;
            let cols = self.cols;

            let mut links = UnGraphMap::with_capacity(rows * cols, 0);
            for row in 0..rows {
                for col in 0..cols {
                    links.add_node(Self::Cell::new(row as isize, col as isize));
                }
            }

            links
        }
    };
}

macro_rules! default_neighbours {
    () => {
        fn neighbours(grid: &Grid<Self>, cell: Self::Cell) -> impl Iterator<Item = Self::Cell> {
            let north = grid.north(cell);
            let south = grid.south(cell);
            let west = grid.west(cell);
            let east = grid.east(cell);

            [north, south, west, east].into_iter().flatten()
        }
    };
}

impl Kind for Regular {
    type Cell = RegularCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    default_prepare_grid!();
    default_neighbours!();
}

impl Kind for Masked {
    type Cell = RegularCell;

    fn num_rows(&self) -> usize {
        self.0.num_rows()
    }

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
        let mask = &self.0;
        let rows = mask.num_rows();
        let cols = mask.num_cols();

        let mut links = UnGraphMap::with_capacity(rows * cols, 0);
        for row in 0..rows {
            for col in 0..cols {
                if mask[row][col] {
                    links.add_node(RegularCell {
                        row: row as isize,
                        col: col as isize,
                    });
                }
            }
        }

        links
    }

    default_neighbours!();
}

impl Kind for Polar {
    type Cell = PolarCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    fn prepare_grid(&self) -> UnGraphMap<Self::Cell, ()> {
        let rows = self.rows as f32;
        let row_height = 1.0 / rows;

        let mut links = UnGraphMap::new();
        links.add_node(PolarCell { row: 0, col: 0 });

        for row in 1..self.rows {
            let radius = row as f32 / rows;
            let circumference = 2.0 * f32::consts::PI * radius;

            let previous_count = row_len(&links, row as isize - 1);
            let estimated_cell_width = circumference / previous_count as f32;
            let ratio = (estimated_cell_width / row_height).round() as usize;

            let cells = previous_count * ratio;
            for col in 0..cells {
                links.add_node(PolarCell {
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
    type Cell = HexCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    default_prepare_grid!();

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

impl Kind for Triangle {
    type Cell = TriangleCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    default_prepare_grid!();
    default_neighbours!();
}

impl Kind for Weighted {
    type Cell = WeightedCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    default_prepare_grid!();
    default_neighbours!();
}

impl Kind for Weave {
    type Cell = WeaveCell;

    fn num_rows(&self) -> usize {
        self.rows
    }

    default_prepare_grid!();

    fn link(grid: &mut Grid<Self>, cell: Self::Cell, other: Self::Cell) {
        let neighbour = if grid.north(cell).is_some() && grid.north(cell) == grid.south(other) {
            grid.north(cell)
        } else if grid.south(cell).is_some() && grid.south(cell) == grid.north(other) {
            grid.south(cell)
        } else if grid.east(cell).is_some() && grid.east(cell) == grid.west(other) {
            grid.east(cell)
        } else if grid.west(cell).is_some() && grid.west(cell) == grid.east(other) {
            grid.west(cell)
        } else {
            None
        };

        match neighbour {
            Some(WeaveCell::Over(neighbour)) => grid.tunnel_under(cell, neighbour, other),
            Some(WeaveCell::Under(_)) => panic!("cannot tunnel under another under cell"),
            None => {
                grid.connect(cell, other);
            }
        }
    }

    fn neighbours(grid: &Grid<Self>, cell: WeaveCell) -> impl Iterator<Item = WeaveCell> {
        let north = grid.north(cell);
        let south = grid.south(cell);
        let west = grid.west(cell);
        let east = grid.east(cell);

        let north_north = grid
            .north(cell)
            .filter(|north| grid.is_horizontal_passage(*north))
            .and_then(|north| grid.north(north));
        let south_south = grid
            .south(cell)
            .filter(|south| grid.is_horizontal_passage(*south))
            .and_then(|south| grid.south(south));
        let west_west = grid
            .west(cell)
            .filter(|west| grid.is_vertical_passage(*west))
            .and_then(|west| grid.west(west));
        let east_east = grid
            .east(cell)
            .filter(|east| grid.is_vertical_passage(*east))
            .and_then(|east| grid.east(east));

        [
            north,
            south,
            west,
            east,
            north_north,
            south_south,
            west_west,
            east_east,
        ]
        .into_iter()
        .flatten()
    }
}
