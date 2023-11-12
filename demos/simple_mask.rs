fn main() {
    let mut mask = mazes::Mask::new(5, 5);
    mask[0][0] = false;
    mask[0][1] = false;
    mask[1][0] = false;
    mask[2][2] = false;
    mask[4][4] = false;

    let kind = mazes::Masked::new(mask);
    let grid = mazes::Grid::new(kind, None, None).recursive_backtracker();
    println!("{grid}");
}
