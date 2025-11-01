use crate::core::cell::Position;
use crate::core::dense_grid::DenseGrid;
use crate::core::grid::Grid;
use crate::core::rules::Rule;
use crate::core::boundary::Boundary;
use crate::core::neighborhood::Neighborhood;
use crate::error::Result;

/// Main automata simulation engine
///
/// Manages the grid state, rule application, and evolution of the cellular automaton.
pub struct Automata {
    /// Current grid state
    grid: DenseGrid,
    /// Next grid state (for double-buffering)
    next_grid: DenseGrid,
    /// Rule to apply for evolution
    rule: Box<dyn Rule>,
    /// Boundary condition for edge handling
    boundary: Box<dyn Boundary>,
    /// Neighborhood type for neighbor calculation
    neighborhood: Box<dyn Neighborhood>,
    /// Current generation number
    generation: u64,
}

impl Automata {
    /// Creates a new automata instance
    ///
    /// # Arguments
    /// * `width` - Width of the grid
    /// * `height` - Height of the grid
    /// * `rule` - Rule to apply for evolution
    /// * `boundary` - Boundary condition for edge handling
    /// * `neighborhood` - Neighborhood type for neighbor calculation
    pub fn new(
        width: usize,
        height: usize,
        rule: Box<dyn Rule>,
        boundary: Box<dyn Boundary>,
        neighborhood: Box<dyn Neighborhood>,
    ) -> Result<Self> {
        let grid = DenseGrid::new(width, height)?;
        let next_grid = DenseGrid::new(width, height)?;

        Ok(Self {
            grid,
            next_grid,
            rule,
            boundary,
            neighborhood,
            generation: 0,
        })
    }

    /// Returns a reference to the current grid
    pub fn grid(&self) -> &DenseGrid {
        &self.grid
    }

    /// Returns a mutable reference to the current grid
    pub fn grid_mut(&mut self) -> &mut DenseGrid {
        &mut self.grid
    }

    /// Returns the current generation number
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Resets the automata to generation 0 and clears the grid
    pub fn reset(&mut self) {
        self.grid.clear();
        self.next_grid.clear();
        self.generation = 0;
    }

    /// Counts alive neighbors for a given position
    fn count_alive_neighbors(&self, pos: Position) -> usize {
        let mut count = 0;
        let x = pos.x as isize;
        let y = pos.y as isize;

        for &(dx, dy) in self.neighborhood.offsets() {
            let neighbor_x = x + dx;
            let neighbor_y = y + dy;

            if let Some(neighbor_pos) = self.boundary.wrap(
                neighbor_x,
                neighbor_y,
                self.grid.width(),
                self.grid.height(),
            ) {
                if let Ok(state) = self.grid.get(neighbor_pos) {
                    if state.is_alive() {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Advances the simulation by one generation
    ///
    /// Applies the rule to each cell based on its current state and neighbor count,
    /// then swaps the current and next grids.
    pub fn step(&mut self) {
        // Calculate next state for all cells
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let pos = Position::new(x, y);
                let current_state = self.grid.get(pos).unwrap();
                let alive_neighbors = self.count_alive_neighbors(pos);
                let next_state = self.rule.apply(current_state, alive_neighbors);
                self.next_grid.set(pos, next_state).unwrap();
            }
        }

        // Swap grids (double-buffering)
        std::mem::swap(&mut self.grid, &mut self.next_grid);

        // Increment generation counter
        self.generation += 1;
    }

    /// Advances the simulation by multiple generations
    ///
    /// # Arguments
    /// * `steps` - Number of generations to advance
    pub fn step_n(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
}