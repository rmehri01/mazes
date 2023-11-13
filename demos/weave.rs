use mazes::kind;

fn main() {
    let kind = kind::Weave::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None)
        .recursive_backtracker()
        .braid(0.5);
    grid.save_png("weave", 25, 0.1);

    grid.set_start(grid.get_random_cell());
    grid.save_png("weave_colorized", 25, 0.1);
}
