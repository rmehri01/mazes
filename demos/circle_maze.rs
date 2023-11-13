use mazes::{cell, kind};

fn main() {
    let kind = kind::Polar::new(16);
    let mut grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    grid.save_png("circle_maze", 25);

    grid.set_start(cell::PolarCell { row: 0, col: 0 });
    grid.save_png("circle_maze_colorized", 25);
}
