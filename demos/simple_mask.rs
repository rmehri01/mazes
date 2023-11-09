fn main() {
    let mut mask = mazes::Mask::new(5, 5);
    mask[0][0] = false;
    mask[0][1] = false;
    mask[1][0] = false;
    mask[2][2] = false;
    mask[4][4] = false;

    let mut grid = mazes::Grid::new(&mask, None, None);
    mazes::recursive_backtracker(&mut grid);
    println!("{grid}");
}
