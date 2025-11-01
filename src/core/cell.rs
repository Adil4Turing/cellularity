use serde::{Deserialize, Serialize};

/// Represents the state of a cell in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellState {
    /// Cell is alive
    Alive,
    /// Cell is dead
    Dead,
}

impl CellState {
    /// Returns true if the cell is alive
    pub fn is_alive(&self) -> bool {
        matches!(self, CellState::Alive)
    }

    /// Returns true if the cell is dead
    pub fn is_dead(&self) -> bool {
        matches!(self, CellState::Dead)
    }

    /// Toggles the cell state
    pub fn toggle(&self) -> Self {
        match self {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
        }
    }
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Dead
    }
}

/// Represents a position in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate (column)
    pub x: usize,
    /// Y coordinate (row)
    pub y: usize,
}

impl Position {
    /// Creates a new position
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Checks if this position is within the given bounds
    pub fn is_within_bounds(&self, width: usize, height: usize) -> bool {
        self.x < width && self.y < height
    }
}