use mazes::kind;

fn main() {
    let kind = kind::Regular::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).ellers();

    println!("{grid}");
    grid.save_png("ellers", 25, 0.0);

    grid.set_start(grid.get_random_cell());
    println!("{grid}");
    grid.save_png("ellers_colorized", 25, 0.0);
}
