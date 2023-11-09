// TODO: might want to split into generators and solvers
fn main() {
    let mut grid = mazes::Grid::new(16, 16, Some(mazes::Cell { row: 0, col: 0 }), None);
    mazes::binary_tree(&mut grid);
    println!("{grid}");

    grid.set_end(mazes::Cell {
        row: grid.rows() as isize - 1,
        col: 0,
    });
    println!("{grid}");
}
