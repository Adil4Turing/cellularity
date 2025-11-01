use cellularity::{CellState, Position, DenseGrid, Grid, Error};

// Tests derived from src/core/grid.rs (trait behavior via a local mock)
struct MockGrid {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

impl MockGrid {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height, cells: vec![CellState::Dead; width * height] }
    }
}

impl Grid for MockGrid {
    fn get(&self, pos: Position) -> Result<CellState, Error> {
        if !pos.is_within_bounds(self.width, self.height) {
            return Err(Error::OutOfBounds { x: pos.x, y: pos.y, width: self.width, height: self.height });
        }
        Ok(self.cells[pos.y * self.width + pos.x])
    }

    fn set(&mut self, pos: Position, state: CellState) -> Result<(), Error> {
        if !pos.is_within_bounds(self.width, self.height) {
            return Err(Error::OutOfBounds { x: pos.x, y: pos.y, width: self.width, height: self.height });
        }
        let index = pos.y * self.width + pos.x;
        self.cells[index] = state;
        Ok(())
    }

    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn clear(&mut self) { self.cells.fill(CellState::Dead); }
    fn count_alive(&self) -> usize { self.cells.iter().filter(|c| c.is_alive()).count() }
}

#[test]
fn test_grid_dimensions() {
    let grid = MockGrid::new(10, 20);
    assert_eq!(grid.width(), 10);
    assert_eq!(grid.height(), 20);
    assert_eq!(grid.size(), 200);
}

#[test]
fn test_grid_get_set() {
    let mut grid = MockGrid::new(5, 5);
    let pos = Position::new(2, 3);
    assert_eq!(grid.get(pos).unwrap(), CellState::Dead);
    grid.set(pos, CellState::Alive).unwrap();
    assert_eq!(grid.get(pos).unwrap(), CellState::Alive);
}

#[test]
fn test_grid_out_of_bounds() {
    let grid = MockGrid::new(5, 5);
    let pos = Position::new(10, 10);
    assert!(grid.get(pos).is_err());
}

#[test]
fn test_grid_clear() {
    let mut grid = MockGrid::new(5, 5);
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(2, 2), CellState::Alive).unwrap();
    assert_eq!(grid.count_alive(), 2);
    grid.clear();
    assert_eq!(grid.count_alive(), 0);
}

#[test]
fn test_grid_count_alive() {
    let mut grid = MockGrid::new(3, 3);
    assert_eq!(grid.count_alive(), 0);
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(1, 1), CellState::Alive).unwrap();
    grid.set(Position::new(2, 2), CellState::Alive).unwrap();
    assert_eq!(grid.count_alive(), 3);
}

// Tests derived from src/core/dense_grid.rs

#[test]
fn test_new_grid() {
    let grid = DenseGrid::new(10, 20).unwrap();
    assert_eq!(grid.width(), 10);
    assert_eq!(grid.height(), 20);
    assert_eq!(grid.size(), 200);
    assert_eq!(grid.count_alive(), 0);
}

#[test]
fn test_new_grid_invalid_dimensions() {
    assert!(DenseGrid::new(0, 10).is_err());
    assert!(DenseGrid::new(10, 0).is_err());
    assert!(DenseGrid::new(0, 0).is_err());
}

#[test]
fn test_get_set() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    let pos = Position::new(2, 3);
    assert_eq!(grid.get(pos).unwrap(), CellState::Dead);
    grid.set(pos, CellState::Alive).unwrap();
    assert_eq!(grid.get(pos).unwrap(), CellState::Alive);
}

#[test]
fn test_get_out_of_bounds() {
    let grid = DenseGrid::new(5, 5).unwrap();
    assert!(grid.get(Position::new(5, 0)).is_err());
    assert!(grid.get(Position::new(0, 5)).is_err());
    assert!(grid.get(Position::new(10, 10)).is_err());
}

#[test]
fn test_set_out_of_bounds() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    assert!(grid.set(Position::new(5, 0), CellState::Alive).is_err());
    assert!(grid.set(Position::new(0, 5), CellState::Alive).is_err());
}

#[test]
fn test_clear() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(2, 2), CellState::Alive).unwrap();
    grid.set(Position::new(4, 4), CellState::Alive).unwrap();
    assert_eq!(grid.count_alive(), 3);
    grid.clear();
    assert_eq!(grid.count_alive(), 0);
}

#[test]
fn test_resize_grow() {
    let mut grid = DenseGrid::new(3, 3).unwrap();
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(1, 1), CellState::Alive).unwrap();
    grid.set(Position::new(2, 2), CellState::Alive).unwrap();
    grid.resize(5, 5).unwrap();
    assert_eq!(grid.width(), 5);
    assert_eq!(grid.height(), 5);
    assert_eq!(grid.get(Position::new(0, 0)).unwrap(), CellState::Alive);
    assert_eq!(grid.get(Position::new(1, 1)).unwrap(), CellState::Alive);
    assert_eq!(grid.get(Position::new(2, 2)).unwrap(), CellState::Alive);
    assert_eq!(grid.get(Position::new(3, 3)).unwrap(), CellState::Dead);
}

#[test]
fn test_resize_shrink() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(1, 1), CellState::Alive).unwrap();
    grid.set(Position::new(4, 4), CellState::Alive).unwrap();
    grid.resize(3, 3).unwrap();
    assert_eq!(grid.width(), 3);
    assert_eq!(grid.height(), 3);
    assert_eq!(grid.get(Position::new(0, 0)).unwrap(), CellState::Alive);
    assert_eq!(grid.get(Position::new(1, 1)).unwrap(), CellState::Alive);
    assert!(grid.get(Position::new(4, 4)).is_err());
}

#[test]
fn test_resize_invalid_dimensions() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    assert!(grid.resize(0, 5).is_err());
    assert!(grid.resize(5, 0).is_err());
}

#[test]
fn test_new_random() {
    let grid = DenseGrid::new_random(10, 10, 0.5).unwrap();
    let alive_count = grid.count_alive();
    assert!(alive_count >= 30 && alive_count <= 70);
}

#[test]
fn test_new_random_zero_probability() {
    let grid = DenseGrid::new_random(10, 10, 0.0).unwrap();
    assert_eq!(grid.count_alive(), 0);
}

#[test]
fn test_new_random_one_probability() {
    let grid = DenseGrid::new_random(10, 10, 1.0).unwrap();
    assert_eq!(grid.count_alive(), 100);
}

#[test]
fn test_iter() {
    let mut grid = DenseGrid::new(3, 3).unwrap();
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    grid.set(Position::new(1, 1), CellState::Alive).unwrap();
    let alive_positions: Vec<Position> = grid
        .iter()
        .filter(|(_, state)| state.is_alive())
        .map(|(pos, _)| pos)
        .collect();
    assert_eq!(alive_positions.len(), 2);
    assert!(alive_positions.contains(&Position::new(0, 0)));
    assert!(alive_positions.contains(&Position::new(1, 1)));
}

#[test]
fn test_count_alive() {
    let mut grid = DenseGrid::new(5, 5).unwrap();
    assert_eq!(grid.count_alive(), 0);
    grid.set(Position::new(0, 0), CellState::Alive).unwrap();
    assert_eq!(grid.count_alive(), 1);
    grid.set(Position::new(1, 1), CellState::Alive).unwrap();
    grid.set(Position::new(2, 2), CellState::Alive).unwrap();
    assert_eq!(grid.count_alive(), 3);
    grid.set(Position::new(0, 0), CellState::Dead).unwrap();
    assert_eq!(grid.count_alive(), 2);
}


