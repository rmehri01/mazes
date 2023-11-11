use rand::seq::SliceRandom;

use crate::grid::{Grid, Regular};

pub fn sidewinder(grid: &mut Grid<Regular>) {
    for row in grid.rows() {
        let mut run = Vec::new();

        for cell in row {
            run.push(cell);

            let at_east_boundary = grid.east(cell).is_none();
            let at_north_boundary = grid.north(cell).is_none();

            let should_close = at_east_boundary || (!at_north_boundary && rand::random());

            if should_close {
                let member = run
                    .choose(&mut rand::thread_rng())
                    .copied()
                    .expect("run should be non-empty");

                if let Some(north) = grid.north(member) {
                    grid.link(member, north);
                }
                run.clear();
            } else {
                let east = grid
                    .east(cell)
                    .expect("east should always exist when not closing a run");
                grid.link(cell, east);
            }
        }
    }
}
