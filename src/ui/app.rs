use eframe::egui;
use eframe::egui::Color32;

use crate::core::cell::{CellState, Position};
use crate::core::dense_grid::DenseGrid;
use crate::core::grid::Grid;

use super::components::grid_view::grid_view;

pub struct CellularityApp {
    grid: DenseGrid,
    generation: usize,
    cell_size: f32,
    alive_color: Color32,
    dead_color: Color32,
}

impl CellularityApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // default small grid for initial visual sanity
        let mut grid = DenseGrid::new(50, 30).expect("valid dimensions");

        // seed a simple blinker
        let _ = grid.set(Position::new(10, 10), CellState::Alive);
        let _ = grid.set(Position::new(11, 10), CellState::Alive);
        let _ = grid.set(Position::new(12, 10), CellState::Alive);

        Self {
            grid,
            generation: 0,
            cell_size: 16.0,
            alive_color: Color32::from_rgb(60, 220, 120),
            dead_color: Color32::from_rgb(30, 30, 35),
        }
    }
}

impl eframe::App for CellularityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Generation: {}", self.generation));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            grid_view(ui, &self.grid, self.cell_size, self.alive_color, self.dead_color);
        });
    }
}

