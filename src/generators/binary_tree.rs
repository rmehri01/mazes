use rand::Rng;

use crate::{
    grid::Grid,
    kind::{Hex, Regular, ThreeD},
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

impl Grid<ThreeD> {
    pub fn binary_tree(mut self) -> Self {
        for cell in self.cells() {
            match (self.north(cell), self.east(cell), self.up(cell)) {
                (None, None, None) => {}
                (None, Some(other), None)
                | (Some(other), None, None)
                | (None, None, Some(other)) => self.link(cell, other),
                (None, Some(first), Some(second))
                | (Some(first), None, Some(second))
                | (Some(first), Some(second), None) => {
                    if rand::random() {
                        self.link(cell, first);
                    } else {
                        self.link(cell, second);
                    }
                }
                (Some(north), Some(east), Some(up)) => match rand::thread_rng().gen_range(0..3) {
                    0 => self.link(cell, north),
                    1 => self.link(cell, east),
                    2 => self.link(cell, up),
                    _ => unreachable!(),
                },
            }
        }

        self
    }
}
