use core::fmt;

use petgraph::prelude::UnGraphMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    row: isize,
    col: isize,
}

pub struct Grid {
    rows: usize,
    cols: usize,
    links: UnGraphMap<Cell, ()>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut links = UnGraphMap::with_capacity(rows * cols, 0);
        for row in 0..rows as isize {
            for col in 0..cols as isize {
                links.add_node(Cell { row, col });
            }
        }

        Self { rows, cols, links }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn iter_cells(rows: usize, cols: usize) -> impl Iterator<Item = Cell> {
        (0..rows as isize).flat_map(move |row| (0..cols as isize).map(move |col| Cell { row, col }))
    }

    pub fn iter_rows(rows: usize, cols: usize) -> impl Iterator<Item = impl Iterator<Item = Cell>> {
        (0..rows as isize).map(move |row| (0..cols as isize).map(move |col| Cell { row, col }))
    }

    pub fn link(&mut self, cell: Cell, other: Cell) {
        self.links.add_edge(cell, other, ());
    }
    pub fn unlink(&mut self, cell: Cell, other: Cell) {
        self.links.remove_edge(cell, other);
    }

    pub fn links(&self, cell: Cell) {
        todo!()
    }
    pub fn are_linked(&self, cell: Cell, other: Cell) -> bool {
        self.links.contains_edge(cell, other)
    }

    pub fn neighbours(&self, cell: Cell) {
        todo!()
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
        todo!()
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
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
                (true, true, true, true) => {
                    unreachable!("not possible in a perfect maze")
                }
            }
        };

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
                // TODO: may be none later
                let cell = self.get(row, col).unwrap();

                top.push_str("   ");
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
