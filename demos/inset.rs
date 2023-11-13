use mazes::{cell, kind};

fn main() {
    let kind = kind::Regular::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    println!("{grid}");
    grid.save_png("inset", 25, 0.1);

    grid.set_start(cell::RegularCell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("inset_colorized", 25, 0.1);
}
