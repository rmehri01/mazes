use mazes::kind;

fn main() {
    let kind = kind::Weave::new(25, 25);
    let mut grid = mazes::Grid::new(kind, None, None).kruskals_better_weave();
    grid.save_png("kruskals", 25, 0.2);

    grid.set_start(grid.get_random_cell());
    grid.save_png("kruskals_colorized", 25, 0.2);
}
