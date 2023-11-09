fn main() {
    let generators = [
        "binary_tree",
        "sidewinder",
        "aldous_broder",
        "wilsons",
        "hunt_and_kill",
        "recursive_backtracker",
    ];

    const TRIES: usize = 100;
    const SIZE: usize = 20;

    let mut averages = generators.map(|name| {
        println!("running {name}");

        let deadend_counts = (0..TRIES)
            .map(|_| {
                let mut grid = mazes::Grid::new(SIZE, SIZE, None, None);
                // TODO: could clean up
                let generator = match name {
                    "binary_tree" => mazes::binary_tree,
                    "sidewinder" => mazes::sidewinder,
                    "aldous_broder" => mazes::aldous_broder,
                    "wilsons" => mazes::wilsons,
                    "hunt_and_kill" => mazes::hunt_and_kill,
                    "recursive_backtracker" => mazes::recursive_backtracker,
                    _ => panic!("invalid generator"),
                };
                generator(&mut grid);
                grid.dead_ends().count()
            })
            .collect::<Vec<_>>();

        let average = deadend_counts.iter().sum::<usize>() as f32 / deadend_counts.len() as f32;
        (name, average)
    });
    averages.sort_by(|(_, avg1), (_, avg2)| avg2.partial_cmp(avg1).unwrap());

    let total_cells = SIZE * SIZE;
    println!("\nAverage dead-ends per {SIZE}x{SIZE} maze ({total_cells} cells):");
    for (name, avg) in averages {
        let percentage = avg * 100.0 / (SIZE * SIZE) as f32;
        println!("{name:>22} : {avg:>5.1}/{total_cells} ({percentage:.1}%)");
    }
}
