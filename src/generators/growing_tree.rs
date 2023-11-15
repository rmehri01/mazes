use rand::seq::IteratorRandom;

use crate::{grid::Grid, kind::Kind};

impl<K: Kind> Grid<K> {
    pub fn growing_tree(mut self, choose: fn(&Vec<K::Cell>) -> Option<&K::Cell>) -> Self {
        let start = self.get_random_cell();
        let mut active = vec![start];

        while let Some(&cell) = choose(&active) {
            let available_neighbours = self
                .neighbours(cell)
                .filter(|n| self.links(*n).next().is_none());

            match available_neighbours.choose(&mut rand::thread_rng()) {
                Some(neighbour) => {
                    self.link(cell, neighbour);
                    active.push(neighbour);
                }
                None => {
                    let idx = active
                        .iter()
                        .position(|c| *c == cell)
                        .expect("cell to be found");
                    active.swap_remove(idx);
                }
            }
        }

        self
    }
}
