use rand::seq::IteratorRandom;

use crate::grid::{Grid, GridKind};

pub fn aldous_broder(grid: &mut Grid<impl GridKind>) {
    let mut cell = grid.get_random_cell();
    let mut unvisited = grid.size() - 1;

    while unvisited > 0 {
        let neighbour = grid
            .neighbours(cell)
            .choose(&mut rand::thread_rng())
            .expect("neighbours should be non-empty");

        if grid.links(neighbour).next().is_none() {
            grid.link(cell, neighbour);
            unvisited -= 1;
        }

        cell = neighbour;
    }
}
