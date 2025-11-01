use crate::core::boundary::Boundary;
use crate::core::cell::Position;

/// Toroidal boundary - edges wrap around to opposite sides
///
/// This creates a "donut" topology where:
/// - Moving off the right edge wraps to the left edge
/// - Moving off the left edge wraps to the right edge
/// - Moving off the top edge wraps to the bottom edge
/// - Moving off the bottom edge wraps to the top edge
#[derive(Debug, Clone, Copy)]
pub struct ToroidalBoundary;

impl ToroidalBoundary {
    /// Creates a new toroidal boundary
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToroidalBoundary {
    fn default() -> Self {
        Self::new()
    }
}

impl Boundary for ToroidalBoundary {
    fn wrap(&self, x: isize, y: isize, width: usize, height: usize) -> Option<Position> {
        if width == 0 || height == 0 {
            return None;
        }

        let width_i = width as isize;
        let height_i = height as isize;

        // Use modulo to wrap coordinates
        // Add width/height before modulo to handle negative numbers correctly
        let wrapped_x = ((x % width_i) + width_i) % width_i;
        let wrapped_y = ((y % height_i) + height_i) % height_i;

        Some(Position::new(wrapped_x as usize, wrapped_y as usize))
    }
}