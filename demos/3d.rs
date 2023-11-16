use mazes::kind;

fn main() {
    let kind = kind::ThreeD::new(5, 5, 3);
    let mut grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    grid.save_png("3d", 50, 0.0);

    grid.set_start(grid.get_random_cell());
    grid.save_png("3d_colorized", 50, 0.0);
}
