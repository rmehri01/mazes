use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::{grid::Grid, kind::Kind};

impl<K: Kind> Grid<K> {
    pub fn wilsons(mut self) -> Self {
        let mut unvisited = self.cells();

        let first_idx = rand::thread_rng().gen_range(0..unvisited.len());
        unvisited.swap_remove(first_idx);

        while let Some(mut cell) = unvisited.choose(&mut rand::thread_rng()).copied() {
            let mut path = vec![cell];

            while unvisited.contains(&cell) {
                cell = self
                    .neighbours(cell)
                    .choose(&mut rand::thread_rng())
                    .expect("neighbours should be non-empty");
                let position = path.iter().position(|c| *c == cell);

                match position {
                    Some(pos) => {
                        path.drain(pos + 1..);
                    }
                    None => path.push(cell),
                }
            }

            for idx in 0..=path.len() - 2 {
                self.link(path[idx], path[idx + 1]);
                unvisited.swap_remove(
                    unvisited
                        .iter()
                        .position(|c| *c == path[idx])
                        .expect("cell should be in the unvisited list"),
                );
            }
        }

        self
    }
}
