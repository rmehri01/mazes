fn main() {
    let kind = mazes::Regular::new(16, 16);
    let mut grid = mazes::Grid::new(kind, None, None);
    mazes::sidewinder(&mut grid);
    println!("{grid}");
    grid.save_png("sidewinder", 25);
}
