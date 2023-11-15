use mazes::kind;

#[derive(Debug)]
enum Generator {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker,
    Kruskals,
    SimplifiedPrims,
    TruePrims,
}

fn main() {
    let generators = [
        Generator::BinaryTree,
        Generator::Sidewinder,
        Generator::AldousBroder,
        Generator::Wilsons,
        Generator::HuntAndKill,
        Generator::RecursiveBacktracker,
        Generator::Kruskals,
        Generator::SimplifiedPrims,
        Generator::TruePrims,
    ];

    const TRIES: usize = 100;
    const SIZE: usize = 20;

    let mut averages = generators.map(|generator| {
        println!("running {generator:?}");

        let deadend_counts = (0..TRIES)
            .map(|_| {
                let kind = kind::Regular::new(SIZE, SIZE);
                let grid = mazes::Grid::new(kind, None, None);
                let grid = match generator {
                    Generator::BinaryTree => grid.binary_tree(),
                    Generator::Sidewinder => grid.sidewinder(),
                    Generator::AldousBroder => grid.aldous_broder(),
                    Generator::Wilsons => grid.wilsons(),
                    Generator::HuntAndKill => grid.hunt_and_kill(),
                    Generator::RecursiveBacktracker => grid.recursive_backtracker(),
                    Generator::Kruskals => grid.kruskals(),
                    Generator::SimplifiedPrims => grid.simplified_prims(),
                    Generator::TruePrims => grid.true_prims(),
                };
                grid.dead_ends().len()
            })
            .collect::<Vec<_>>();

        let average = deadend_counts.iter().sum::<usize>() as f32 / deadend_counts.len() as f32;
        (generator, average)
    });
    averages.sort_by(|(_, avg1), (_, avg2)| avg2.partial_cmp(avg1).unwrap());

    let total_cells = SIZE * SIZE;
    println!("\nAverage dead-ends per {SIZE}x{SIZE} maze ({total_cells} cells):");
    for (generator, avg) in averages {
        let percentage = avg * 100.0 / (SIZE * SIZE) as f32;
        println!(
            "{:<20} : {avg:>5.1}/{total_cells} ({percentage:.1}%)",
            format!("{generator:?}")
        );
    }
}
