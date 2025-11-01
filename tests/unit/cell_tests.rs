use cellularity::{CellState, Position};

#[test]
fn test_cell_state_alive() {
    let cell = CellState::Alive;
    assert!(cell.is_alive());
    assert!(!cell.is_dead());
}

#[test]
fn test_cell_state_dead() {
    let cell = CellState::Dead;
    assert!(!cell.is_alive());
    assert!(cell.is_dead());
}

#[test]
fn test_cell_state_toggle() {
    let alive = CellState::Alive;
    assert_eq!(alive.toggle(), CellState::Dead);

    let dead = CellState::Dead;
    assert_eq!(dead.toggle(), CellState::Alive);
}

#[test]
fn test_cell_state_default() {
    let cell: CellState = Default::default();
    assert_eq!(cell, CellState::Dead);
}

#[test]
fn test_position_new() {
    let pos = Position::new(5, 10);
    assert_eq!(pos.x, 5);
    assert_eq!(pos.y, 10);
}

#[test]
fn test_position_within_bounds() {
    let pos = Position::new(5, 10);
    assert!(pos.is_within_bounds(10, 20));
    assert!(!pos.is_within_bounds(5, 10));
    assert!(!pos.is_within_bounds(3, 20));
}

#[test]
fn test_position_equality() {
    let pos1 = Position::new(5, 10);
    let pos2 = Position::new(5, 10);
    let pos3 = Position::new(6, 10);

    assert_eq!(pos1, pos2);
    assert_ne!(pos1, pos3);
}


