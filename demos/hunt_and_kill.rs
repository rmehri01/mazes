use mazes::kind;

fn main() {
    let kind = kind::Regular::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).hunt_and_kill();
    println!("{grid}");
    grid.save_png("hunt_and_kill", 25);

    grid.set_start(mazes::RegularCell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("hunt_and_kill_colorized", 25);
}
