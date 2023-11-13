use mazes::{cell, kind};

fn main() {
    let kind = kind::Triangle::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    grid.save_png("triangle_maze", 25);

    grid.set_start(cell::TriangleCell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    grid.save_png("triangle_maze_colorized", 25);
}
