fn main() {
    let mut grid = mazes::Grid::new(16, 16);
    mazes::binary_tree(&mut grid);
    println!("{grid}");
    grid.save_png("binary_tree", 10);
}
