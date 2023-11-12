use mazes::kind;
use rand::seq::IteratorRandom;

fn main() {
    let kind = kind::Weighted { rows: 10, cols: 10 };
    let mut grid = mazes::Grid::new(kind, None, None)
        .recursive_backtracker()
        .braid(0.5);

    grid.set_start(grid.get(0, 0).unwrap());
    println!("{grid}");
    grid.set_goal(
        grid.get(grid.num_rows() as isize - 1, grid.num_cols() as isize - 1)
            .unwrap(),
    );
    grid.save_png("weighted_original", 25, 0.0);
    println!("{grid}");

    let cell = grid
        .distances()
        .unwrap()
        .cells()
        .choose(&mut rand::thread_rng())
        .expect("at least one cell in the path");
    grid.set_weight(cell.row, cell.col, 50);
    grid.save_png("weighted_rerouted", 25, 0.0);
    println!("{grid}");
}
