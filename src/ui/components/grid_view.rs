use eframe::egui;
use eframe::egui::{Color32, Pos2, Rect, Vec2};

use crate::core::cell::CellState;
use crate::core::dense_grid::DenseGrid;
use crate::core::grid::Grid;

pub fn grid_view(
    ui: &mut egui::Ui,
    grid: &DenseGrid,
    cell_size: f32,
    alive: Color32,
    dead: Color32,
) {
    let available = ui.available_rect_before_wrap();
    let origin = available.min;
    let painter = ui.painter_at(available);

    for y in 0..grid.height() as usize {
        for x in 0..grid.width() as usize {
            let state = grid
                .get(crate::core::cell::Position::new(x, y))
                .unwrap_or(CellState::Dead);
            let color = state_to_color(state, alive, dead);
            let rect = grid_to_rect(x as f32, y as f32, cell_size, origin);
            painter.rect_filled(rect, 0.0, color);
        }
    }

    // reserve space so egui layout accounts for our drawing
    let total_size = Vec2::new(grid.width() as f32 * cell_size, grid.height() as f32 * cell_size);
    ui.allocate_space(total_size);
}

pub fn state_to_color(state: CellState, alive: Color32, dead: Color32) -> Color32 {
    match state {
        CellState::Alive => alive,
        CellState::Dead => dead,
    }
}

pub fn grid_to_rect(x: f32, y: f32, cell_size: f32, origin: Pos2) -> Rect {
    let min = Pos2::new(origin.x + x * cell_size, origin.y + y * cell_size);
    let max = Pos2::new(min.x + cell_size, min.y + cell_size);
    Rect { min, max }
}

