use crate::core::cell::CellState;
use crate::core::rules::trait_def::Rule;

/// Conway's Game of Life rule 
///
/// Rules:
/// - Any live cell with 2 or 3 live neighbors survives
/// - Any dead cell with exactly 3 live neighbors becomes alive
/// - All other cells die or stay dead
#[derive(Debug, Clone, Copy)]
pub struct ConwayRule;

impl ConwayRule {
    /// Creates a new Conway's Game of Life rule
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConwayRule {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ConwayRule {
    fn apply(&self, current_state: CellState, alive_neighbors: usize) -> CellState {
        match (current_state, alive_neighbors) {
            // Birth: dead cell with exactly 3 neighbors becomes alive
            (CellState::Dead, 3) => CellState::Alive,
            // Survival: alive cell with 2 or 3 neighbors stays alive
            (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
            // Death: all other cases result in dead cell
            _ => CellState::Dead,
        }
    }

    fn name(&self) -> &str {
        "Conway's Game of Life"
    }

    fn description(&self) -> &str {
        "B3/S23 - Birth on 3 neighbors, Survival on 2 or 3 neighbors"
    }
}