use crate::core::neighborhood::Neighborhood;

/// Moore neighborhood - includes all 8 surrounding cells
///
/// Pattern:
/// ```text
/// X X X
/// X O X
/// X X X
/// ```
/// Where O is the center cell and X are neighbors
#[derive(Debug, Clone, Copy)]
pub struct MooreNeighborhood {
    offsets: [(isize, isize); 8],
}

impl MooreNeighborhood {
    /// Creates a new Moore neighborhood
    pub fn new() -> Self {
        Self {
            offsets: [
                (-1, -1), (0, -1), (1, -1),  // Top row
                (-1,  0),          (1,  0),  // Middle row (excluding center)
                (-1,  1), (0,  1), (1,  1),  // Bottom row
            ],
        }
    }
}

impl Default for MooreNeighborhood {
    fn default() -> Self {
        Self::new()
    }
}

impl Neighborhood for MooreNeighborhood {
    fn offsets(&self) -> &[(isize, isize)] {
        &self.offsets
    }

    fn name(&self) -> &str {
        "Moore (8 neighbors)"
    }
}