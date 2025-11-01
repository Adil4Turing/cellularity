use crate::core::cell::{CellState, Position};
use crate::error::Result;

/// Trait defining the interface for grid implementations
pub trait Grid {
    /// Gets the cell state at the given position
    fn get(&self, pos: Position) -> Result<CellState>;

    /// Sets the cell state at the given position
    fn set(&mut self, pos: Position, state: CellState) -> Result<()>;

    /// Returns the width of the grid
    fn width(&self) -> usize;

    /// Returns the height of the grid
    fn height(&self) -> usize;

    /// Returns the total number of cells in the grid
    fn size(&self) -> usize {
        self.width() * self.height()
    }

    /// Clears the grid (sets all cells to dead)
    fn clear(&mut self);

    /// Counts the number of alive cells in the grid
    fn count_alive(&self) -> usize;
}