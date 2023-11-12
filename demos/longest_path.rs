use mazes::kind;

fn main() {
    let kind = kind::Regular::new(16, 16);
    let mut grid = mazes::Grid::new(kind, None, None).binary_tree();

    let (new_start, _) = grid.distances_from(mazes::Cell { row: 0, col: 0 }).max();
    grid.set_start(new_start);

    let (goal, _) = grid.distances_from(new_start).max();
    grid.set_goal(goal);

    println!("{grid}");
}
