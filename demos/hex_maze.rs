use mazes::kind;

fn main() {
    let kind = kind::Hex::new(20, 20);
    let mut grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    grid.save_png("hex_maze", 25);

    grid.set_start(mazes::HexCell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    grid.save_png("hex_maze_colorized", 25);
}
