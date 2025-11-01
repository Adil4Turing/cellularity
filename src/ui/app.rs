use eframe::egui;
use eframe::egui::Color32;

use crate::core::cell::{CellState, Position};
use crate::core::automata::Automata;
use crate::core::grid::Grid;
use crate::core::rules::ConwayRule;
use crate::core::toroidal_boundary::ToroidalBoundary;
use crate::core::moore_neighborhood::MooreNeighborhood;

use super::components::{grid_view::grid_view, ControlPanel, ControlAction};

pub struct CellularityApp {
    automata: Automata,
    control_panel: ControlPanel,
    cell_size: f32,
    alive_color: Color32,
    dead_color: Color32,
    // Simulation timing
    last_update: std::time::Instant,
    update_interval: std::time::Duration,
}

impl CellularityApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Create automata with initial setup
        let mut automata = Automata::new(
            50,  // width
            30,  // height
            Box::new(ConwayRule::new()),
            Box::new(ToroidalBoundary::new()),
            Box::new(MooreNeighborhood::new()),
        ).expect("valid automata parameters");

        // Seed a simple blinker pattern
        let _ = automata.grid_mut().set(Position::new(10, 10), CellState::Alive);
        let _ = automata.grid_mut().set(Position::new(11, 10), CellState::Alive);
        let _ = automata.grid_mut().set(Position::new(12, 10), CellState::Alive);

        Self {
            automata,
            control_panel: ControlPanel::new(0),
            cell_size: 16.0,
            alive_color: Color32::from_rgb(60, 220, 120),
            dead_color: Color32::from_rgb(30, 30, 35),
            last_update: std::time::Instant::now(),
            update_interval: std::time::Duration::from_millis(100), // 10 steps per second
        }
    }
}

impl eframe::App for CellularityApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the generation in the control panel
        self.control_panel.generation = self.automata.generation();

        // Show the control panel in top panel
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            match self.control_panel.show(ui) {
                ControlAction::Play => {
                    self.control_panel.is_playing = true;
                },
                ControlAction::Pause => {
                    self.control_panel.is_playing = false;
                },
                ControlAction::Step => {
                    self.automata.step();
                },
                ControlAction::Reset => {
                    self.automata.reset();
                    self.control_panel.is_playing = false;
                },
                ControlAction::None => {},
            }
        });

        // Handle automatic simulation updates if playing
        if self.control_panel.is_playing {
            let elapsed = self.last_update.elapsed();
            if elapsed >= self.update_interval {
                self.automata.step();
                self.last_update = std::time::Instant::now();
                
                // Request repaint to keep the simulation running
                ctx.request_repaint();
            }
        }

        // Show the grid in central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            grid_view(ui, self.automata.grid(), self.cell_size, self.alive_color, self.dead_color);
        });
    }
}

