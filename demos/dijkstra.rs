// TODO: might want to split into generators and solvers
fn main() {
    let mut grid = mazes::Grid::new(
        &mazes::Mask::new(16, 16),
        Some(mazes::Cell { row: 0, col: 0 }),
        None,
    );
    mazes::binary_tree(&mut grid);
    println!("{grid}");

    grid.set_goal(mazes::Cell {
        row: grid.num_rows() as isize - 1,
        col: 0,
    });
    println!("{grid}");
}
