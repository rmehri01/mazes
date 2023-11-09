fn main() {
    let mut grid = mazes::Grid::new(16, 16, None, None);
    mazes::binary_tree(&mut grid);

    let (new_start, _) = grid.distances_from(mazes::Cell { row: 0, col: 0 }).max();
    grid.set_start(new_start);

    let (goal, _) = grid.distances_from(new_start).max();
    grid.set_goal(goal);

    println!("{grid}");
}
