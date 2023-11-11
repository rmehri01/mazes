fn main() {
    let mask = mazes::Mask::from_txt("masks/ascii.txt");
    let kind = mazes::Masked::new(mask);
    let mut grid = mazes::Grid::new(kind, None, None);
    mazes::recursive_backtracker(&mut grid);

    println!("{grid}");
    grid.save_png("ascii_mask", 25);
}
