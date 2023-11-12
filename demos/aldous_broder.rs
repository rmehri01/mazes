use mazes::kind;

fn main() {
    let kind = kind::Regular::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).aldous_broder();
    println!("{grid}");
    grid.save_png("aldous_broder", 25, 0.0);

    grid.set_start(mazes::RegularCell {
        row: grid.num_rows() as isize / 2,
        col: grid.num_cols() as isize / 2,
    });
    println!("{grid}");
    grid.save_png("aldous_broder_colorized", 25, 0.0);
}
