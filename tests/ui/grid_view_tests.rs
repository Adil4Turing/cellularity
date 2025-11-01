use cellularity::ui::components::grid_view::{grid_to_rect, state_to_color};
use cellularity::core::cell::CellState;
use eframe::egui::{pos2, Color32};

#[test]
fn grid_to_rect_maps_cells_correctly() {
    let origin = pos2(2.5, 3.5);
    let cell_size = 8.0;

    let r00 = grid_to_rect(0.0, 0.0, cell_size, origin);
    assert_eq!(r00.min.x, 2.5);
    assert_eq!(r00.min.y, 3.5);
    assert_eq!(r00.max.x, 10.5);
    assert_eq!(r00.max.y, 11.5);

    let r12 = grid_to_rect(1.0, 2.0, cell_size, origin);
    assert_eq!(r12.min.x, 2.5 + 1.0 * cell_size);
    assert_eq!(r12.min.y, 3.5 + 2.0 * cell_size);
    assert_eq!(r12.max.x, r12.min.x + cell_size);
    assert_eq!(r12.max.y, r12.min.y + cell_size);
}

#[test]
fn state_to_color_matches_mapping() {
    let alive = Color32::from_rgb(1, 2, 3);
    let dead = Color32::from_rgb(4, 5, 6);
    assert_eq!(state_to_color(CellState::Alive, alive, dead), alive);
    assert_eq!(state_to_color(CellState::Dead, alive, dead), dead);
}

