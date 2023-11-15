use mazes::kind;

fn main() {
    let kind = kind::Regular::new(25, 25);
    let grid = mazes::Grid::new(kind, None, None).simplified_prims();

    println!("{grid}");
    grid.save_png("prims_simplified", 25, 0.0);

    let kind = kind::Regular::new(25, 25);
    let grid = mazes::Grid::new(kind, None, None).true_prims();

    println!("{grid}");
    grid.save_png("prims_true", 25, 0.0);
}
