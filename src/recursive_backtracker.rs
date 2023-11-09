use rand::seq::IteratorRandom;

use crate::grid::Grid;

pub fn recursive_backtracker(grid: &mut Grid) {
    let mut stack = vec![grid.get_random_cell()];

    while let Some(current) = stack.last() {
        let neighbours = grid
            .neighbours(*current)
            .filter(|n| grid.links(*n).next().is_none());

        match neighbours.choose(&mut rand::thread_rng()) {
            Some(neighbour) => {
                grid.link(*current, neighbour);
                stack.push(neighbour);
            }
            None => {
                stack.pop();
            }
        }
    }
}
