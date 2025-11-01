use cellularity::{Rule, ConwayRule, CellState};

// Tests derived from src/core/rules/trait_def.rs using a local mock
struct MockRule;

impl Rule for MockRule {
    fn apply(&self, current_state: CellState, alive_neighbors: usize) -> CellState {
        if alive_neighbors == 3 { CellState::Alive } else { current_state }
    }
    fn name(&self) -> &str { "Mock Rule" }
}

#[test]
fn test_rule_trait_implementation() {
    let rule = MockRule;
    assert_eq!(rule.name(), "Mock Rule");
    assert_eq!(rule.apply(CellState::Dead, 3), CellState::Alive);
    assert_eq!(rule.apply(CellState::Alive, 2), CellState::Alive);
}

#[test]
fn test_rule_description_default() {
    let rule = MockRule;
    assert_eq!(rule.description(), "No description available");
}

// Tests derived from src/core/rules/conway.rs
#[test]
fn test_conway_rule_birth() {
    let rule = ConwayRule::new();
    assert_eq!(rule.apply(CellState::Dead, 3), CellState::Alive);
}

#[test]
fn test_conway_rule_survival() {
    let rule = ConwayRule::new();
    assert_eq!(rule.apply(CellState::Alive, 2), CellState::Alive);
    assert_eq!(rule.apply(CellState::Alive, 3), CellState::Alive);
}

#[test]
fn test_conway_rule_death() {
    let rule = ConwayRule::new();
    assert_eq!(rule.apply(CellState::Alive, 0), CellState::Dead);
    assert_eq!(rule.apply(CellState::Alive, 1), CellState::Dead);
    assert_eq!(rule.apply(CellState::Alive, 4), CellState::Dead);
    assert_eq!(rule.apply(CellState::Alive, 8), CellState::Dead);
}

#[test]
fn test_conway_rule_dead_stays_dead() {
    let rule = ConwayRule::new();
    for neighbors in 0..=2 {
        assert_eq!(rule.apply(CellState::Dead, neighbors), CellState::Dead);
    }
    for neighbors in 4..=8 {
        assert_eq!(rule.apply(CellState::Dead, neighbors), CellState::Dead);
    }
}

#[test]
fn test_conway_rule_name() {
    let rule = ConwayRule::new();
    assert_eq!(rule.name(), "Conway's Game of Life");
}

#[test]
fn test_conway_rule_description() {
    let rule = ConwayRule::new();
    assert!(rule.description().contains("B3/S23"));
}

#[test]
fn test_conway_rule_default() {
    let rule: ConwayRule = Default::default();
    assert_eq!(rule.name(), "Conway's Game of Life");
}


