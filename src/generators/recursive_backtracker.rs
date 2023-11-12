use rand::seq::IteratorRandom;

use crate::{grid::Grid, kind::Kind};

impl<K: Kind> Grid<K> {
    pub fn recursive_backtracker(mut self) -> Self {
        let mut stack = vec![self.get_random_cell()];

        while let Some(current) = stack.last() {
            let neighbours = self
                .neighbours(*current)
                .filter(|n| self.links(*n).next().is_none());

            match neighbours.choose(&mut rand::thread_rng()) {
                Some(neighbour) => {
                    self.link(*current, neighbour);
                    stack.push(neighbour);
                }
                None => {
                    stack.pop();
                }
            }
        }

        self
    }
}
