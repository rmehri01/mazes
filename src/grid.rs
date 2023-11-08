use core::fmt;

use petgraph::prelude::UnGraphMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    row: usize,
    col: usize,
}

pub struct Grid {
    rows: usize,
    cols: usize,
    links: UnGraphMap<Cell, ()>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut links = UnGraphMap::with_capacity(rows * cols, 0);
        for row in 0..rows {
            for col in 0..cols {
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
        (0..rows).flat_map(move |row| (0..cols).map(move |col| Cell { row, col }))
    }

    pub fn iter_rows(rows: usize, cols: usize) -> impl Iterator<Item = impl Iterator<Item = Cell>> {
        (0..rows).map(move |row| (0..cols).map(move |col| Cell { row, col }))
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
        // TODO: better to use isize?
        if cell.row == 0 {
            return None;
        }

        self.get(cell.row - 1, cell.col)
    }
    pub fn south(&self, cell: Cell) -> Option<Cell> {
        self.get(cell.row + 1, cell.col)
    }
    pub fn west(&self, cell: Cell) -> Option<Cell> {
        if cell.col == 0 {
            return None;
        }

        self.get(cell.row, cell.col - 1)
    }
    pub fn east(&self, cell: Cell) -> Option<Cell> {
        self.get(cell.row, cell.col + 1)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
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
        writeln!(f, "+{}", "---+".repeat(self.cols))?;

        for row in 0..self.rows {
            let mut top = "|".to_string();
            let mut bot = "+".to_string();

            for col in 0..self.cols {
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
                    '|'
                };
                top.push(east_boundary);

                let south_boundary = if self
                    .south(cell)
                    .map(|south| self.are_linked(cell, south))
                    .unwrap_or(false)
                {
                    "   "
                } else {
                    "---"
                };
                bot.push_str(south_boundary);
                bot.push('+');
            }

            writeln!(f, "{top}")?;
            writeln!(f, "{bot}")?;
        }

        Ok(())
    }
}
