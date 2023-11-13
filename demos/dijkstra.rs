use mazes::{cell, kind};

fn main() {
    let kind = kind::Regular::new(16, 16);
    let mut grid =
        mazes::Grid::new(kind, Some(cell::RegularCell { row: 0, col: 0 }), None).binary_tree();
    println!("{grid}");

    grid.set_goal(cell::RegularCell {
        row: grid.num_rows() as isize - 1,
        col: 0,
    });
    println!("{grid}");
}
