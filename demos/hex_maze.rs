fn main() {
    let kind = mazes::Hex::new(20, 20);
    let mut grid = mazes::Grid::new(kind, None, None).sidewinder();
    grid.save_png("hex_maze", 25);

    grid.set_start(mazes::Cell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    grid.save_png("hex_maze_colorized", 25);
}
