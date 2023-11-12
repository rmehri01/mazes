fn main() {
    let mask = mazes::Mask::from_image("masks/image.png");
    let kind = mazes::Masked::new(mask);
    let grid = mazes::Grid::new(kind, None, None).recursive_backtracker();

    println!("{grid}");
    grid.save_png("image_mask", 25);
}
