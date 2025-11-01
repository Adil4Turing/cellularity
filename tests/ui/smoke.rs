// Smoke test: basic types link and helper functions are callable without panic.
#[test]
fn ui_smoke_compiles_and_helpers_ok() {
    use cellularity::ui::components::grid_view::{grid_to_rect, state_to_color};
    use cellularity::core::cell::CellState;
    use eframe::egui::{pos2, Color32};

    let rect = grid_to_rect(0.0, 0.0, 10.0, pos2(5.0, 7.0));
    assert_eq!(rect.min.x, 5.0);
    assert_eq!(rect.min.y, 7.0);

    let c_alive = state_to_color(CellState::Alive, Color32::WHITE, Color32::BLACK);
    let c_dead = state_to_color(CellState::Dead, Color32::WHITE, Color32::BLACK);
    assert_eq!(c_alive, Color32::WHITE);
    assert_eq!(c_dead, Color32::BLACK);
}

