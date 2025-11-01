use crate::core::neighborhood::Neighborhood;

/// Von Neumann neighborhood - includes only orthogonally adjacent cells
///
/// Pattern:
/// ```text
///   X
/// X O X
///   X
/// ```
/// Where O is the center cell and X are neighbors
#[derive(Debug, Clone, Copy)]
pub struct VonNeumannNeighborhood {
    offsets: [(isize, isize); 4],
}

impl VonNeumannNeighborhood {
    /// Creates a new Von Neumann neighborhood
    pub fn new() -> Self {
        Self {
            offsets: [
                (0, -1),  // Top
                (-1, 0),  // Left
                (1, 0),   // Right
                (0, 1),   // Bottom
            ],
        }
    }
}

impl Default for VonNeumannNeighborhood {
    fn default() -> Self {
        Self::new()
    }
}

impl Neighborhood for VonNeumannNeighborhood {
    fn offsets(&self) -> &[(isize, isize)] {
        &self.offsets
    }

    fn name(&self) -> &str {
        "Von Neumann (4 neighbors)"
    }
}