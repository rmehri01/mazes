fn main() {
    let kind = mazes::Regular::new(16, 16);
    let mut grid = mazes::Grid::new(kind, None, None);
    mazes::binary_tree(&mut grid);
    println!("{grid}");
    grid.save_png("binary_tree", 25);
}
