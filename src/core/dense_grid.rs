use crate::core::cell::{CellState, Position};
use crate::core::grid::Grid;
use crate::error::{Error, Result};
use rand::Rng;

/// Dense grid implementation using a flat vector
#[derive(Debug, Clone)]
pub struct DenseGrid {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

impl DenseGrid {
    /// Creates a new grid with the specified dimensions
    ///
    /// # Arguments
    /// * `width` - Width of the grid
    /// * `height` - Height of the grid
    ///
    /// # Returns
    /// A new grid with all cells initialized to dead
    pub fn new(width: usize, height: usize) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::InvalidDimensions { width, height });
        }

        Ok(Self {
            cells: vec![CellState::Dead; width * height],
            width,
            height,
        })
    }

    /// Creates a new grid with random initial state
    ///
    /// # Arguments
    /// * `width` - Width of the grid
    /// * `height` - Height of the grid
    /// * `alive_probability` - Probability (0.0 to 1.0) that a cell is alive
    pub fn new_random(width: usize, height: usize, alive_probability: f64) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::InvalidDimensions { width, height });
        }

        let mut rng = rand::rng();
        let cells = (0..width * height)
            .map(|_| {
                if rng.random::<f64>() < alive_probability {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            })
            .collect();

        Ok(Self {
            cells,
            width,
            height,
        })
    }

    /// Resizes the grid, preserving existing cells where possible
    ///
    /// # Arguments
    /// * `new_width` - New width of the grid
    /// * `new_height` - New height of the grid
    pub fn resize(&mut self, new_width: usize, new_height: usize) -> Result<()> {
        if new_width == 0 || new_height == 0 {
            return Err(Error::InvalidDimensions {
                width: new_width,
                height: new_height,
            });
        }

        let mut new_cells = vec![CellState::Dead; new_width * new_height];

        // Copy existing cells that fit in the new dimensions
        let copy_width = self.width.min(new_width);
        let copy_height = self.height.min(new_height);

        for y in 0..copy_height {
            for x in 0..copy_width {
                let old_index = y * self.width + x;
                let new_index = y * new_width + x;
                new_cells[new_index] = self.cells[old_index];
            }
        }

        self.cells = new_cells;
        self.width = new_width;
        self.height = new_height;

        Ok(())
    }

    /// Converts a position to an index in the flat vector
    fn pos_to_index(&self, pos: Position) -> Result<usize> {
        if !pos.is_within_bounds(self.width, self.height) {
            return Err(Error::OutOfBounds {
                x: pos.x,
                y: pos.y,
                width: self.width,
                height: self.height,
            });
        }
        Ok(pos.y * self.width + pos.x)
    }

    /// Returns an iterator over all positions and their cell states
    pub fn iter(&self) -> impl Iterator<Item = (Position, CellState)> + '_ {
        self.cells.iter().enumerate().map(|(index, &state)| {
            let x = index % self.width;
            let y = index / self.width;
            (Position::new(x, y), state)
        })
    }
}

impl Grid for DenseGrid {
    fn get(&self, pos: Position) -> Result<CellState> {
        let index = self.pos_to_index(pos)?;
        Ok(self.cells[index])
    }

    fn set(&mut self, pos: Position, state: CellState) -> Result<()> {
        let index = self.pos_to_index(pos)?;
        self.cells[index] = state;
        Ok(())
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn clear(&mut self) {
        self.cells.fill(CellState::Dead);
    }

    fn count_alive(&self) -> usize {
        self.cells.iter().filter(|c| c.is_alive()).count()
    }
}