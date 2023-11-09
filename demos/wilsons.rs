fn main() {
    let mut grid = mazes::Grid::new(&mazes::Mask::new(25, 25), None, None);
    mazes::wilsons(&mut grid);
    println!("{grid}");
    grid.save_png("wilsons", 25);

    grid.set_start(mazes::Cell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("wilsons_colorized", 25);
}
