pub mod core;
pub mod error;
pub mod utils;

// Re-export commonly used types
pub use core::{
    automata::Automata,
    boundary::Boundary,
    cell::{CellState, Position},
    dense_grid::DenseGrid,
    grid::Grid,
    moore_neighborhood::MooreNeighborhood,
    neighborhood::Neighborhood,
    rules::{ConwayRule, Rule},
    toroidal_boundary::ToroidalBoundary,
    von_neumann_neighborhood::VonNeumannNeighborhood,
    walled_boundary::WalledBoundary,
};
pub use error::{Error, Result};