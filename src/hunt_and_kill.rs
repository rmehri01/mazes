use rand::seq::IteratorRandom;

use crate::grid::Grid;

pub fn hunt_and_kill(grid: &mut Grid) {
    let mut current = Some(grid.get_random_cell());

    while let Some(cell) = current {
        let unvisited_neighbours = grid
            .neighbours(cell)
            .filter(|n| grid.links(*n).next().is_none());

        match unvisited_neighbours.choose(&mut rand::thread_rng()) {
            Some(neighbour) => {
                grid.link(cell, neighbour);
                current = Some(neighbour);
            }
            None => {
                current = Grid::iter_cells(grid.rows(), grid.cols()).find_map(|c| {
                    if grid.links(c).next().is_none() {
                        let visited_neighbours = grid
                            .neighbours(c)
                            .filter(|n| grid.links(*n).next().is_some());

                        visited_neighbours
                            .choose(&mut rand::thread_rng())
                            .map(|neighbour| {
                                grid.link(c, neighbour);
                                c
                            })
                    } else {
                        None
                    }
                });
            }
        }
    }
}
