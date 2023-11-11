fn main() {
    let mut grid = mazes::Grid::new(mazes::Polar::new(8), None, None);
    mazes::recursive_backtracker(&mut grid);
    grid.save_png("circle_maze", 25);
}
