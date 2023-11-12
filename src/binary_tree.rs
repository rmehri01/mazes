use crate::{
    grid::{Grid, Regular},
    Hex,
};

impl Grid<Regular> {
    pub fn binary_tree(mut self) -> Self {
        for cell in self.cells() {
            match (self.north(cell), self.east(cell)) {
                (None, None) => {}
                (None, Some(other)) | (Some(other), None) => self.link(cell, other),
                (Some(north), Some(east)) => {
                    if rand::random() {
                        self.link(cell, north);
                    } else {
                        self.link(cell, east);
                    }
                }
            }
        }

        self
    }
}

impl Grid<Hex> {
    pub fn binary_tree(mut self) -> Self {
        for cell in self.cells() {
            match (self.north(cell), self.get_next_in_row(cell)) {
                (None, None) => {}
                (None, Some(other)) | (Some(other), None) => self.link(cell, other),
                (Some(north), Some(east)) => {
                    if rand::random() {
                        self.link(cell, north);
                    } else {
                        self.link(cell, east);
                    }
                }
            }
        }

        self
    }
}
