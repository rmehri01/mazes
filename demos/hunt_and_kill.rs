fn main() {
    let mut grid = mazes::Grid::new(25, 25, None, None);
    mazes::hunt_and_kill(&mut grid);
    println!("{grid}");
    grid.save_png("hunt_and_kill", 25);

    grid.set_start(mazes::Cell {
        row: grid.rows() as isize / 2,
        col: grid.cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("hunt_and_kill_colorized", 25);
}
