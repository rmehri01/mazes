fn main() {
    let kind = mazes::Regular::new(16, 16);
    let grid = mazes::Grid::new(kind, None, None).binary_tree();
    println!("{grid}");
    grid.save_png("binary_tree", 25);
}
