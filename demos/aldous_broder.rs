fn main() {
    let mut grid = mazes::Grid::new(25, 25, None, None);
    mazes::aldous_broder(&mut grid);
    println!("{grid}");
    grid.save_png("aldous_broder", 25);

    grid.set_start(mazes::Cell {
        row: grid.rows() as isize / 2,
        col: grid.cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("aldous_broder_colorized", 25);
}
