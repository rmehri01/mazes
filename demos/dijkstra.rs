use mazes::kind;

// TODO: might want to split into generators and solvers
fn main() {
    let kind = kind::Regular::new(16, 16);
    let mut grid =
        mazes::Grid::new(kind, Some(mazes::RegularCell { row: 0, col: 0 }), None).binary_tree();
    println!("{grid}");

    grid.set_goal(mazes::RegularCell {
        row: grid.num_rows() as isize - 1,
        col: 0,
    });
    println!("{grid}");
}
