fn main() {
    let mask = mazes::Mask::from_image("masks/image.png");
    let kind = mazes::Masked::new(mask);
    let mut grid = mazes::Grid::new(kind, None, None);
    mazes::recursive_backtracker(&mut grid);

    println!("{grid}");
    grid.save_png("image_mask", 25);
}
