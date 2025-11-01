use cellularity::{
    Automata, CellState, ConwayRule, DenseGrid, Grid, MooreNeighborhood, Position,
    ToroidalBoundary,
};

fn main() {
    println!("Cellular Automata Simulator");
    println!("==================================\n");

    // Create a simple automata instance
    let mut automata = Automata::new(
        10,
        10,
        Box::new(ConwayRule::new()),
        Box::new(ToroidalBoundary::new()),
        Box::new(MooreNeighborhood::new()),
    )
    .expect("Failed to create automata");

    // Set up a blinker pattern
    println!("Setting up a blinker pattern...");
    automata
        .grid_mut()
        .set(Position::new(4, 4), CellState::Alive)
        .unwrap();
    automata
        .grid_mut()
        .set(Position::new(5, 4), CellState::Alive)
        .unwrap();
    automata
        .grid_mut()
        .set(Position::new(6, 4), CellState::Alive)
        .unwrap();

    println!("Initial state:");
    print_grid(automata.grid());

    // Run simulation for a few steps
    for i in 1..=4 {
        automata.step();
        println!("\nGeneration {}:", i);
        print_grid(automata.grid());
    }

    println!("\n✓ core functionality is working!");
  
}

fn print_grid(grid: &DenseGrid) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let pos = Position::new(x, y);
            let cell = grid.get(pos).unwrap();
            print!("{}", if cell.is_alive() { "█" } else { "·" });
        }
        println!();
    }
}
