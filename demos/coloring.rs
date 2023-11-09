fn main() {
    const ROWS: usize = 25;
    const COLS: usize = 25;

    let mut grid = mazes::Grid::new(
        ROWS,
        COLS,
        Some(mazes::Cell {
            row: ROWS as isize / 2,
            col: COLS as isize / 2,
        }),
        None,
    );
    mazes::binary_tree(&mut grid);
    println!("{grid}");
    grid.save_png("colorized", 25);
}
