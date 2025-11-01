use cellularity::{Boundary, ToroidalBoundary, WalledBoundary, Position};

// Tests derived from src/core/boundary.rs using a local mock
struct MockBoundary;

impl Boundary for MockBoundary {
    fn wrap(&self, x: isize, y: isize, width: usize, height: usize) -> Option<Position> {
        if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
            Some(Position::new(x as usize, y as usize))
        } else {
            None
        }
    }
}

#[test]
fn test_boundary_trait() {
    let boundary = MockBoundary;
    assert_eq!(boundary.wrap(5, 5, 10, 10), Some(Position::new(5, 5)));
    assert_eq!(boundary.wrap(-1, 5, 10, 10), None);
    assert_eq!(boundary.wrap(5, -1, 10, 10), None);
    assert_eq!(boundary.wrap(10, 5, 10, 10), None);
    assert_eq!(boundary.wrap(5, 10, 10, 10), None);
}

// Toroidal boundary tests
#[test]
fn test_toroidal_boundary_within_bounds() {
    let boundary = ToroidalBoundary::new();
    let pos = boundary.wrap(5, 5, 10, 10).unwrap();
    assert_eq!(pos, Position::new(5, 5));
}

#[test]
fn test_toroidal_boundary_wrap_right() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(10, 5, 10, 10).unwrap(), Position::new(0, 5));
    assert_eq!(boundary.wrap(11, 5, 10, 10).unwrap(), Position::new(1, 5));
}

#[test]
fn test_toroidal_boundary_wrap_left() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(-1, 5, 10, 10).unwrap(), Position::new(9, 5));
    assert_eq!(boundary.wrap(-2, 5, 10, 10).unwrap(), Position::new(8, 5));
}

#[test]
fn test_toroidal_boundary_wrap_bottom() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(5, 10, 10, 10).unwrap(), Position::new(5, 0));
    assert_eq!(boundary.wrap(5, 11, 10, 10).unwrap(), Position::new(5, 1));
}

#[test]
fn test_toroidal_boundary_wrap_top() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(5, -1, 10, 10).unwrap(), Position::new(5, 9));
    assert_eq!(boundary.wrap(5, -2, 10, 10).unwrap(), Position::new(5, 8));
}

#[test]
fn test_toroidal_boundary_wrap_corner() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(-1, -1, 10, 10).unwrap(), Position::new(9, 9));
    assert_eq!(boundary.wrap(10, 10, 10, 10).unwrap(), Position::new(0, 0));
}

#[test]
fn test_toroidal_boundary_large_offsets() {
    let boundary = ToroidalBoundary::new();
    assert_eq!(boundary.wrap(25, 5, 10, 10).unwrap(), Position::new(5, 5));
    assert_eq!(boundary.wrap(-15, 5, 10, 10).unwrap(), Position::new(5, 5));
}

#[test]
fn test_toroidal_boundary_zero_dimensions() {
    let boundary = ToroidalBoundary::new();
    assert!(boundary.wrap(5, 5, 0, 10).is_none());
    assert!(boundary.wrap(5, 5, 10, 0).is_none());
    assert!(boundary.wrap(5, 5, 0, 0).is_none());
}

#[test]
fn test_toroidal_boundary_default() {
    let boundary: ToroidalBoundary = Default::default();
    let pos = boundary.wrap(5, 5, 10, 10).unwrap();
    assert_eq!(pos, Position::new(5, 5));
}

// Walled boundary tests
#[test]
fn test_walled_boundary_within_bounds() {
    let boundary = WalledBoundary::new();
    let pos = boundary.wrap(5, 5, 10, 10).unwrap();
    assert_eq!(pos, Position::new(5, 5));
}

#[test]
fn test_walled_boundary_clamp_right() {
    let boundary = WalledBoundary::new();
    assert_eq!(boundary.wrap(10, 5, 10, 10).unwrap(), Position::new(9, 5));
    assert_eq!(boundary.wrap(100, 5, 10, 10).unwrap(), Position::new(9, 5));
}

#[test]
fn test_walled_boundary_clamp_bottom() {
    let boundary = WalledBoundary::new();
    assert_eq!(boundary.wrap(5, 10, 10, 10).unwrap(), Position::new(5, 9));
    assert_eq!(boundary.wrap(5, 100, 10, 10).unwrap(), Position::new(5, 9));
}

#[test]
fn test_walled_boundary_negative_x() {
    let boundary = WalledBoundary::new();
    assert!(boundary.wrap(-1, 5, 10, 10).is_none());
    assert!(boundary.wrap(-10, 5, 10, 10).is_none());
}

#[test]
fn test_walled_boundary_negative_y() {
    let boundary = WalledBoundary::new();
    assert!(boundary.wrap(5, -1, 10, 10).is_none());
    assert!(boundary.wrap(5, -10, 10, 10).is_none());
}

#[test]
fn test_walled_boundary_negative_both() {
    let boundary = WalledBoundary::new();
    assert!(boundary.wrap(-1, -1, 10, 10).is_none());
}

#[test]
fn test_walled_boundary_corner_clamp() {
    let boundary = WalledBoundary::new();
    assert_eq!(boundary.wrap(10, 10, 10, 10).unwrap(), Position::new(9, 9));
    assert_eq!(boundary.wrap(0, 0, 10, 10).unwrap(), Position::new(0, 0));
}

#[test]
fn test_walled_boundary_edge_cases() {
    let boundary = WalledBoundary::new();
    assert_eq!(boundary.wrap(9, 9, 10, 10).unwrap(), Position::new(9, 9));
    assert_eq!(boundary.wrap(10, 9, 10, 10).unwrap(), Position::new(9, 9));
}

#[test]
fn test_walled_boundary_zero_dimensions() {
    let boundary = WalledBoundary::new();
    assert!(boundary.wrap(5, 5, 0, 10).is_none());
    assert!(boundary.wrap(5, 5, 10, 0).is_none());
    assert!(boundary.wrap(5, 5, 0, 0).is_none());
}

#[test]
fn test_walled_boundary_default() {
    let boundary: WalledBoundary = Default::default();
    let pos = boundary.wrap(5, 5, 10, 10).unwrap();
    assert_eq!(pos, Position::new(5, 5));
}

#[test]
fn test_walled_boundary_single_cell() {
    let boundary = WalledBoundary::new();
    assert_eq!(boundary.wrap(0, 0, 1, 1).unwrap(), Position::new(0, 0));
    assert_eq!(boundary.wrap(5, 5, 1, 1).unwrap(), Position::new(0, 0));
}


