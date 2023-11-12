fn main() {
    let mask = mazes::Mask::from_txt("masks/ascii.txt");
    let kind = mazes::Masked::new(mask);
    let grid = mazes::Grid::new(kind, None, None).recursive_backtracker();

    println!("{grid}");
    grid.save_png("ascii_mask", 25);
}
