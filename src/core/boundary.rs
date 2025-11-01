use crate::core::cell::Position;

/// Trait defining boundary behavior for grid coordinates
pub trait Boundary: Send + Sync {
    /// Wraps a position according to the boundary rules
    ///
    /// # Arguments
    /// * `x` - X coordinate (can be negative)
    /// * `y` - Y coordinate (can be negative)
    /// * `width` - Grid width
    /// * `height` - Grid height
    ///
    /// # Returns
    /// A valid position within the grid bounds, or None if out of bounds
    fn wrap(&self, x: isize, y: isize, width: usize, height: usize) -> Option<Position>;
}