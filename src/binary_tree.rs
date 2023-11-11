use crate::grid::{Grid, Regular};

// TODO: should this be part of grid?
pub fn binary_tree(grid: &mut Grid<Regular>) {
    for cell in grid.cells() {
        match (grid.north(cell), grid.east(cell)) {
            (None, None) => {}
            (None, Some(other)) | (Some(other), None) => grid.link(cell, other),
            (Some(north), Some(east)) => {
                if rand::random() {
                    grid.link(cell, north);
                } else {
                    grid.link(cell, east);
                }
            }
        }
    }
}
