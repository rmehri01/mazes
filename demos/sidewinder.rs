fn main() {
    let mut grid = mazes::Grid::new(4, 4);
    mazes::sidewinder(&mut grid);
    println!("{grid}");
}
