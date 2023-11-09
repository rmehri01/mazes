fn main() {
    let mut grid = mazes::Grid::new(&mazes::Mask::new(16, 16), None, None);
    mazes::binary_tree(&mut grid);
    println!("{grid}");
    grid.save_png("binary_tree", 25);
}
