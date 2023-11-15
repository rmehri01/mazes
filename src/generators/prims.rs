use rand::{seq::IteratorRandom, Rng};
use rustc_hash::FxHashMap;

use crate::{grid::Grid, kind::Kind};

impl<K: Kind> Grid<K> {
    pub fn simplified_prims(mut self) -> Self {
        let start = self.get_random_cell();
        let mut active = vec![start];

        while let Some((idx, &cell)) = active.iter().enumerate().choose(&mut rand::thread_rng()) {
            let available_neighbours = self
                .neighbours(cell)
                .filter(|n| self.links(*n).next().is_none());

            match available_neighbours.choose(&mut rand::thread_rng()) {
                Some(neighbour) => {
                    self.link(cell, neighbour);
                    active.push(neighbour);
                }
                None => {
                    active.swap_remove(idx);
                }
            }
        }

        self
    }

    pub fn true_prims(mut self) -> Self {
        let start = self.get_random_cell();
        let mut active = vec![start];

        let costs = FxHashMap::from_iter(
            self.cells()
                .into_iter()
                .map(|cell| (cell, rand::thread_rng().gen_range(0..100))),
        );

        while let Some((idx, &cell)) = active.iter().enumerate().min_by_key(|(_, c)| costs[c]) {
            let available_neighbours = self
                .neighbours(cell)
                .filter(|n| self.links(*n).next().is_none());

            match available_neighbours.min_by_key(|n| costs[n]) {
                Some(neighbour) => {
                    self.link(cell, neighbour);
                    active.push(neighbour);
                }
                None => {
                    active.swap_remove(idx);
                }
            }
        }

        self
    }
}
