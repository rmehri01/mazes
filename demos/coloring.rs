use mazes::{cell, kind};

fn main() {
    const ROWS: usize = 25;
    const COLS: usize = 25;

    let kind = kind::Regular::new(ROWS, COLS);
    let grid = mazes::Grid::new(
        kind,
        Some(cell::RegularCell {
            row: ROWS as isize / 2,
            col: COLS as isize / 2,
        }),
        None,
    )
    .binary_tree();
    println!("{grid}");
    grid.save_png("colorized", 25, 0.0);
}
