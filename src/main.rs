fn main() {
    let mut grid = mazes::Grid::new(4, 4);
    mazes::binary_tree(&mut grid);
    println!("{grid}");
}
