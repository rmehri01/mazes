fn main() {
    let mut grid = mazes::Grid::new(16, 16);
    mazes::sidewinder(&mut grid);
    println!("{grid}");
    grid.save_png("sidewinder", 10);
}
