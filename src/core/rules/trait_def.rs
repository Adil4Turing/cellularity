use crate::core::cell::CellState;

/// Trait defining the interface for cellular automata rules
pub trait Rule: Send + Sync {
    /// Applies the rule to determine the next state of a cell
    ///
    /// # Arguments
    /// * `current_state` - The current state of the cell
    /// * `alive_neighbors` - The number of alive neighbors
    ///
    /// # Returns
    /// The next state of the cell
    fn apply(&self, current_state: CellState, alive_neighbors: usize) -> CellState;

    /// Returns the name of the rule
    fn name(&self) -> &str;

    /// Returns a description of the rule
    fn description(&self) -> &str {
        "No description available"
    }
}