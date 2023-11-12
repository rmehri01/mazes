use rand::seq::IteratorRandom;

use crate::{grid::Grid, kind::Kind};

impl<K: Kind> Grid<K> {
    pub fn hunt_and_kill(mut self) -> Self {
        let mut current = Some(self.get_random_cell());

        while let Some(cell) = current {
            let unvisited_neighbours = self
                .neighbours(cell)
                .filter(|n| self.links(*n).next().is_none());

            match unvisited_neighbours.choose(&mut rand::thread_rng()) {
                Some(neighbour) => {
                    self.link(cell, neighbour);
                    current = Some(neighbour);
                }
                None => {
                    current = self.cells().into_iter().find_map(|c| {
                        if self.links(c).next().is_none() {
                            let visited_neighbours = self
                                .neighbours(c)
                                .filter(|n| self.links(*n).next().is_some());

                            visited_neighbours
                                .choose(&mut rand::thread_rng())
                                .map(|neighbour| {
                                    self.link(c, neighbour);
                                    c
                                })
                        } else {
                            None
                        }
                    });
                }
            }
        }

        self
    }
}
