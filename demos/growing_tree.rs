use mazes::kind;
use rand::seq::SliceRandom;

fn main() {
    let kind = kind::Regular::new(25, 25);
    let grid = mazes::Grid::new(kind, None, None)
        .growing_tree(|list| list.choose(&mut rand::thread_rng()));

    println!("{grid}");
    grid.save_png("growing_tree_random", 25, 0.0);

    let kind = kind::Regular::new(25, 25);
    let grid = mazes::Grid::new(kind, None, None).growing_tree(|list| list.last());

    println!("{grid}");
    grid.save_png("growing_tree_last", 25, 0.0);

    let kind = kind::Regular::new(25, 25);
    let grid = mazes::Grid::new(kind, None, None).growing_tree(|list| {
        if rand::random() {
            list.last()
        } else {
            list.choose(&mut rand::thread_rng())
        }
    });

    println!("{grid}");
    grid.save_png("growing_tree_mix", 25, 0.0);
}
