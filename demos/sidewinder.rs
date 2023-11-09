fn main() {
    let mut grid = mazes::Grid::new(16, 16, None, None);
    mazes::sidewinder(&mut grid);
    println!("{grid}");
    grid.save_png("sidewinder", 25);
}
