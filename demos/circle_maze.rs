fn main() {
    let mut grid = mazes::Grid::new(mazes::Polar::new(16), None, None).recursive_backtracker();
    grid.save_png("circle_maze", 25);

    grid.set_start(mazes::Cell { row: 0, col: 0 });
    grid.save_png("circle_maze_colorized", 25);
}
