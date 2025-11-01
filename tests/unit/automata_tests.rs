use cellularity::{
    Automata, ConwayRule, ToroidalBoundary, MooreNeighborhood, CellState, Position, Grid,
};

fn create_test_automata(width: usize, height: usize) -> Automata {
    Automata::new(
        width,
        height,
        Box::new(ConwayRule::new()),
        Box::new(ToroidalBoundary::new()),
        Box::new(MooreNeighborhood::new()),
    )
    .unwrap()
}

#[test]
fn test_automata_creation() {
    let automata = create_test_automata(10, 10);
    assert_eq!(automata.grid().width(), 10);
    assert_eq!(automata.grid().height(), 10);
    assert_eq!(automata.generation(), 0);
}

#[test]
fn test_automata_reset() {
    let mut automata = create_test_automata(5, 5);
    automata.grid_mut().set(Position::new(2, 2), CellState::Alive).unwrap();
    automata.step();
    assert_eq!(automata.generation(), 1);
    automata.reset();
    assert_eq!(automata.generation(), 0);
    assert_eq!(automata.grid().count_alive(), 0);
}

#[test]
fn test_automata_step() {
    let mut automata = create_test_automata(5, 5);
    assert_eq!(automata.generation(), 0);
    automata.step();
    assert_eq!(automata.generation(), 1);
    automata.step();
    assert_eq!(automata.generation(), 2);
}

#[test]
fn test_automata_step_n() {
    let mut automata = create_test_automata(5, 5);
    automata.step_n(10);
    assert_eq!(automata.generation(), 10);
}

#[test]
fn test_blinker_pattern() {
    let mut automata = create_test_automata(5, 5);
    // Horizontal blinker
    automata.grid_mut().set(Position::new(2, 1), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(2, 2), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(2, 3), CellState::Alive).unwrap();
    assert_eq!(automata.grid().count_alive(), 3);
    // After one step -> vertical
    automata.step();
    assert_eq!(automata.grid().count_alive(), 3);
    assert_eq!(automata.grid().get(Position::new(1, 2)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(2, 2)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(3, 2)).unwrap(), CellState::Alive);
    // After another step -> horizontal
    automata.step();
    assert_eq!(automata.grid().count_alive(), 3);
    assert_eq!(automata.grid().get(Position::new(2, 1)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(2, 2)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(2, 3)).unwrap(), CellState::Alive);
}

#[test]
fn test_block_pattern() {
    let mut automata = create_test_automata(5, 5);
    // 2x2 block
    automata.grid_mut().set(Position::new(1, 1), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(2, 1), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(1, 2), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(2, 2), CellState::Alive).unwrap();
    assert_eq!(automata.grid().count_alive(), 4);
    automata.step();
    assert_eq!(automata.grid().count_alive(), 4);
    assert_eq!(automata.grid().get(Position::new(1, 1)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(2, 1)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(1, 2)).unwrap(), CellState::Alive);
    assert_eq!(automata.grid().get(Position::new(2, 2)).unwrap(), CellState::Alive);
    automata.step_n(10);
    assert_eq!(automata.grid().count_alive(), 4);
}

#[test]
fn test_plus_neighbors_behavior() {
    // Replacement for private neighbor count test: a plus shape around center has 4 neighbors
    // Under Conway's rules and Moore neighborhood, center should remain dead after one step.
    let mut automata = create_test_automata(5, 5);
    let center = Position::new(2, 2);
    automata.grid_mut().set(Position::new(2, 1), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(1, 2), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(3, 2), CellState::Alive).unwrap();
    automata.grid_mut().set(Position::new(2, 3), CellState::Alive).unwrap();
    automata.step();
    assert_eq!(automata.grid().get(center).unwrap(), CellState::Dead);
}

#[test]
fn test_empty_grid_stays_empty() {
    let mut automata = create_test_automata(5, 5);
    assert_eq!(automata.grid().count_alive(), 0);
    automata.step_n(10);
    assert_eq!(automata.grid().count_alive(), 0);
}


