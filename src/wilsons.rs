use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};

use crate::grid::Grid;

pub fn wilsons(grid: &mut Grid) {
    let mut unvisited = Grid::iter_cells(grid.rows(), grid.cols()).collect::<Vec<_>>();

    let first_idx = rand::thread_rng().gen_range(0..unvisited.len());
    unvisited.swap_remove(first_idx);

    while let Some(mut cell) = unvisited.choose(&mut rand::thread_rng()).copied() {
        let mut path = vec![cell];

        while unvisited.contains(&cell) {
            cell = grid
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
            grid.link(path[idx], path[idx + 1]);
            unvisited.swap_remove(
                unvisited
                    .iter()
                    .position(|c| *c == path[idx])
                    .expect("cell should be in the unvisited list"),
            );
        }
    }
}
