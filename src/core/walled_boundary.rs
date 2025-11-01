use crate::core::boundary::Boundary;
use crate::core::cell::Position;

/// Walled boundary - coordinates are clamped to grid edges
///
/// This creates a "box" topology where:
/// - Coordinates beyond the right edge are clamped to the rightmost column
/// - Coordinates beyond the left edge are clamped to the leftmost column
/// - Coordinates beyond the bottom edge are clamped to the bottom row
/// - Coordinates beyond the top edge are clamped to the top row
/// - Negative coordinates return None (out of bounds)
#[derive(Debug, Clone, Copy)]
pub struct WalledBoundary;

impl WalledBoundary {
    /// Creates a new walled boundary
    pub fn new() -> Self {
        Self
    }
}

impl Default for WalledBoundary {
    fn default() -> Self {
        Self::new()
    }
}

impl Boundary for WalledBoundary {
    fn wrap(&self, x: isize, y: isize, width: usize, height: usize) -> Option<Position> {
        if width == 0 || height == 0 {
            return None;
        }

        // Return None for negative coordinates (out of bounds)
        if x < 0 || y < 0 {
            return None;
        }

        // Clamp coordinates to grid bounds
        let clamped_x = (x as usize).min(width - 1);
        let clamped_y = (y as usize).min(height - 1);

        Some(Position::new(clamped_x, clamped_y))
    }
}