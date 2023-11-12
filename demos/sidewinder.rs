fn main() {
    let kind = mazes::Regular::new(16, 16);
    let grid = mazes::Grid::new(kind, None, None).sidewinder();
    println!("{grid}");
    grid.save_png("sidewinder", 25);
}
