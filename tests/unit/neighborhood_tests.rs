use cellularity::{Neighborhood, MooreNeighborhood, VonNeumannNeighborhood};

// Tests derived from src/core/neighborhood.rs using a local mock
struct MockNeighborhood { offsets: Vec<(isize, isize)> }

impl Neighborhood for MockNeighborhood {
    fn offsets(&self) -> &[(isize, isize)] { &self.offsets }
    fn name(&self) -> &str { "Mock Neighborhood" }
}

#[test]
fn test_neighborhood_trait() {
    let neighborhood = MockNeighborhood { offsets: vec![(-1, 0), (1, 0), (0, -1), (0, 1)] };
    assert_eq!(neighborhood.offsets().len(), 4);
    assert_eq!(neighborhood.name(), "Mock Neighborhood");
}

#[test]
fn test_neighborhood_offsets() {
    let neighborhood = MockNeighborhood { offsets: vec![(-1, -1), (0, -1), (1, -1)] };
    let offsets = neighborhood.offsets();
    assert_eq!(offsets[0], (-1, -1));
    assert_eq!(offsets[1], (0, -1));
    assert_eq!(offsets[2], (1, -1));
}

// Tests derived from src/core/moore_neighborhood.rs
#[test]
fn test_moore_neighborhood_offsets() {
    let neighborhood = MooreNeighborhood::new();
    let offsets = neighborhood.offsets();
    assert_eq!(offsets.len(), 8);
    assert!(offsets.contains(&(-1, -1)));
    assert!(offsets.contains(&(0, -1)));
    assert!(offsets.contains(&(1, -1)));
    assert!(offsets.contains(&(-1, 0)));
    assert!(offsets.contains(&(1, 0)));
    assert!(offsets.contains(&(-1, 1)));
    assert!(offsets.contains(&(0, 1)));
    assert!(offsets.contains(&(1, 1)));
    assert!(!offsets.contains(&(0, 0)));
}

#[test]
fn test_moore_neighborhood_name() {
    let neighborhood = MooreNeighborhood::new();
    assert_eq!(neighborhood.name(), "Moore (8 neighbors)");
}

#[test]
fn test_moore_neighborhood_default() {
    let neighborhood: MooreNeighborhood = Default::default();
    assert_eq!(neighborhood.offsets().len(), 8);
}

// Tests derived from src/core/von_neumann_neighborhood.rs
#[test]
fn test_von_neumann_neighborhood_offsets() {
    let neighborhood = VonNeumannNeighborhood::new();
    let offsets = neighborhood.offsets();
    assert_eq!(offsets.len(), 4);
    assert!(offsets.contains(&(0, -1)));
    assert!(offsets.contains(&(-1, 0)));
    assert!(offsets.contains(&(1, 0)));
    assert!(offsets.contains(&(0, 1)));
    assert!(!offsets.contains(&(-1, -1)));
    assert!(!offsets.contains(&(1, -1)));
    assert!(!offsets.contains(&(-1, 1)));
    assert!(!offsets.contains(&(1, 1)));
    assert!(!offsets.contains(&(0, 0)));
}

#[test]
fn test_von_neumann_neighborhood_name() {
    let neighborhood = VonNeumannNeighborhood::new();
    assert_eq!(neighborhood.name(), "Von Neumann (4 neighbors)");
}

#[test]
fn test_von_neumann_neighborhood_default() {
    let neighborhood: VonNeumannNeighborhood = Default::default();
    assert_eq!(neighborhood.offsets().len(), 4);
}


