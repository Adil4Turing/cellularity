# 🧬 Cellular Automata Simulator

<p align="center">
  <em>High-performance Cellular Automata Simulator built in Rust, designed for extensibility and speed, Inspired by Conway's Game of Life</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Language">
  <img src="https://img.shields.io/badge/platform-Cross--Platform-green.svg" alt="Platform">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/version-0.1.0-yellow.svg" alt="Version">
</p>

---

## 🚀 Overview

Cellular Automata Simulator is a feature-rich Rust Project designed for simulating various types of cellular automata. The library is built with extensibility in mind, allowing for easy implementation of custom rules, grid topologies, and neighborhood configurations. The primary implementation focuses on Conway's Game of Life, though the architecture supports many other cellular automata models.

### Key Features

- ✅ **Extensible Rule System**: Implement custom cellular automata rules beyond Conway's Game of Life
- ✅ **Flexible Grid Topology**: Support for various grid sizes and configurations
- ✅ **Multiple Boundary Conditions**: Toroidal (wrapping) and walled boundaries
- ✅ **Different Neighborhood Types**: Moore (8 neighbors) and Von Neumann (4 neighbors) neighborhoods
- ✅ **Double-Buffered Architecture**: Efficient grid state transitions
- ✅ **Pattern Support**: Ready-to-use patterns like blinkers, blocks, and gliders
- ✅ **Unit Tested**: Comprehensive test suite ensuring reliability

---

## 📦 Installation & Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/) (1.70 or higher)
- Cargo (comes with Rust)

### Building the Project

```bash
# Clone and build the project
git clone <repository-url>
cd cellularity

# Build in release mode
cargo build --release

# Or run directly
cargo run
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

---

## 🎮 Usage

### Running the Default Simulation

The default simulation demonstrates Conway's Game of Life with a "blinker" pattern:

```bash
cargo run
```

### Example Output

```
Cellular Automata Simulator
==================================

Setting up a blinker pattern...
Initial state:
··········
··········
··········
··········
···███····
··········
··········
··········
··········
··········

Generation 1:
··········
··········
·····█····
·····█····
·····█····
··········
··········
··········
··········
··········

Generation 2:
··········
··········
··········
····████··
··········
··········
··········
··········
··········
··········
```

### Library Usage

You can also use the library in your own projects:

```rust
use cellularity::{
    Automata, CellState, ConwayRule, DenseGrid, Grid, MooreNeighborhood, Position,
    ToroidalBoundary,
};

fn main() {
    // Create a 20x20 automata with Conway's rules
    let mut automata = Automata::new(
        20,
        20,
        Box::new(ConwayRule::new()),
        Box::new(ToroidalBoundary::new()),
        Box::new(MooreNeighborhood::new()),
    ).expect("Failed to create automata");

    // Set up a custom pattern
    automata
        .grid_mut()
        .set(Position::new(5, 5), CellState::Alive)
        .unwrap();

    // Advance the simulation by 10 generations
    automata.step_n(10);

    // Access grid data
    println!("Current generation: {}", automata.generation());
    println!("Alive cells: {}", automata.grid().count_alive());
}
```

---

## 🏗️ Architecture

The library is structured with a clear separation of concerns:

```
cellularity/
├── Cargo.toml              # Project dependencies and metadata
├── README.md              # This file
├── architecture.md        # Detailed architectural documentation
├── src/
│   ├── main.rs            # Default simulation entry point
│   ├── lib.rs             # Public API re-exports
│   ├── error.rs           # Error handling utilities
│   ├── core/              # Core simulation components
│   │   ├── automata.rs    # Main simulation engine
│   │   ├── cell.rs        # Cell state and position definitions
│   │   ├── grid.rs        # Grid trait definition
│   │   ├── dense_grid.rs  # Dense grid implementation
│   │   ├── rules/         # Rule implementations
│   │   │   ├── mod.rs
│   │   │   └── conway_rule.rs
│   │   ├── boundary/      # Boundary condition implementations
│   │   │   ├── mod.rs
│   │   │   ├── toroidal_boundary.rs
│   │   │   └── walled_boundary.rs
│   │   ├── neighborhood/  # Neighborhood configurations
│   │   │   ├── mod.rs
│   │   │   ├── moore_neighborhood.rs
│   │   │   └── von_neumann_neighborhood.rs
│   │   └── automata.rs    # Main automata struct
│   └── utils/             # Utility modules
│       ├── mod.rs
│       └── error.rs
└── target/                # Compiled artifacts (gitignored)
```

### Key Components

- **`Automata`**: Main simulation engine that orchestrates the cellular automaton
- **`Rule`**: Trait for defining state transition rules (e.g., Conway's Game of Life)
- **`Boundary`**: Trait for handling edge behavior (toroidal or walled)
- **`Neighborhood`**: Trait for defining neighbor relationships (Moore or Von Neumann)
- **`Grid`**: Trait for grid implementations with dense grid as default
- **`CellState`**: Enum representing cell states (Alive/Dead)

---

## 🛠️ Extending the Library

### Implementing Custom Rules

To implement a custom rule, implement the `Rule` trait:

```rust
use cellularity::{Rule, CellState};

pub struct CustomRule;

impl CustomRule {
    pub fn new() -> Self {
        Self
    }
}

impl Rule for CustomRule {
    fn apply(&self, current: CellState, alive_neighbors: usize) -> CellState {
        // Implement custom logic here
        if current.is_alive() {
            if alive_neighbors == 2 || alive_neighbors == 3 {
                CellState::Alive // Stay alive
            } else {
                CellState::Dead // Die due to underpopulation or overpopulation
            }
        } else {
            if alive_neighbors == 3 {
                CellState::Alive // Reproduction
            } else {
                CellState::Dead // Remain dead
            }
        }
    }
}
```

### Custom Boundary Conditions

Implement the `Boundary` trait for custom boundary behavior:

```rust
use cellularity::{Boundary, Position};

pub struct CustomBoundary;

impl CustomBoundary {
    pub fn new() -> Self {
        Self
    }
}

impl Boundary for CustomBoundary {
    fn wrap(&self, x: isize, y: isize, width: usize, height: usize) -> Option<Position> {
        // Custom boundary logic
        // Return None for positions that should be treated as "out of bounds"
        // Return Some(position) for valid grid coordinates
        todo!()
    }
}
```

---

## 📚 Examples

### Predefined Patterns

The library supports various interesting patterns:

- **Blinker**: Oscillates between horizontal and vertical lines (period 2)
- **Block**: Still life that remains unchanged
- **Glider**: Pattern that moves diagonally across the grid
- **Random patterns**: Initialize with random cell states

### Advanced Usage

Advanced users can combine different rules, boundaries, and neighborhoods:

```rust
// Create a hexagonal grid simulation with custom rules
let mut hex_automata = Automata::new(
    50,
    50,
    Box::new(CustomRule::new()),
    Box::new(WalledBoundary::new()),
    Box::new(VonNeumannNeighborhood::new()),
).unwrap();
```

---

## 🧪 Testing

The library includes comprehensive tests for all core functionality:

```bash
# Run all tests
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific test
cargo test test_blinker_pattern
```

### Test Coverage

- ✅ Basic automata creation and initialization
- ✅ Grid state management and transitions
- ✅ Rule application and state evolution
- ✅ Pattern behavior (blinker, block, glider)
- ✅ Boundary condition handling
- ✅ Neighborhood calculations
- ✅ Generation counting and reset functionality

---

## 🤝 Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Write comprehensive tests for new features
- Document public APIs with examples
- Maintain backward compatibility when possible

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- John Conway for the original Game of Life concept
- The Rust community for excellent development tools
- All contributors who help improve this library

---

<p align="center">
  Made with ❤️ and 🦀
</p>