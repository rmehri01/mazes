use rand::seq::IteratorRandom;

use crate::grid::{Grid, GridKind};

impl<K: GridKind> Grid<K> {
    pub fn aldous_broder(mut self) -> Self {
        let mut cell = self.get_random_cell();
        let mut unvisited = self.size() - 1;

        while unvisited > 0 {
            let neighbour = self
                .neighbours(cell)
                .choose(&mut rand::thread_rng())
                .expect("neighbours should be non-empty");

            if self.links(neighbour).next().is_none() {
                self.link(cell, neighbour);
                unvisited -= 1;
            }

            cell = neighbour;
        }

        self
    }
}
