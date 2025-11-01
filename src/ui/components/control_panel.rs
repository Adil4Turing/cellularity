use eframe::egui;

/// Control panel for the cellular automata simulator
pub struct ControlPanel {
    /// Whether the simulation is currently running
    pub is_playing: bool,
    /// Current generation count
    pub generation: u64,
}

impl ControlPanel {
    pub fn new(generation: u64) -> Self {
        Self {
            is_playing: false,
            generation,
        }
    }

    /// Show the control panel UI
    pub fn show(&mut self, ui: &mut egui::Ui) -> ControlAction {
        let mut action = ControlAction::None;

        ui.horizontal(|ui| {
            // Play/Pause button
            if self.is_playing {
                if ui.button("⏸ Pause").clicked() {
                    self.is_playing = false;
                    action = ControlAction::Pause;
                }
            } else {
                if ui.button("▶ Play").clicked() {
                    self.is_playing = true;
                    action = ControlAction::Play;
                }
            }

            // Step button
            if ui.button("⏭ Step").clicked() {
                action = ControlAction::Step;
            }

            // Reset button
            if ui.button("⏹ Reset").clicked() {
                action = ControlAction::Reset;
                self.is_playing = false; // Reset also pauses
            }

            // Generation counter
            ui.separator();
            ui.label(format!("Generation: {}", self.generation));
        });

        action
    }
}

/// Actions that can be triggered from the control panel
#[derive(Debug, PartialEq)]
pub enum ControlAction {
    None,
    Play,
    Pause,
    Step,
    Reset,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_panel_initial_state() {
        let control_panel = ControlPanel::new(0);
        assert_eq!(control_panel.is_playing, false);
        assert_eq!(control_panel.generation, 0);
    }

    #[test]
    fn test_control_panel_play_action() {
        let mut control_panel = ControlPanel::new(5);
        assert_eq!(control_panel.is_playing, false);
        
        // Simulate play action
        control_panel.is_playing = true;
        assert_eq!(control_panel.is_playing, true);
    }

    #[test]
    fn test_control_panel_pause_action() {
        let mut control_panel = ControlPanel::new(10);
        control_panel.is_playing = true;
        assert_eq!(control_panel.is_playing, true);
        
        // Simulate pause action
        control_panel.is_playing = false;
        assert_eq!(control_panel.is_playing, false);
    }

    #[test]
    fn test_control_action_enum() {
        assert_eq!(ControlAction::None, ControlAction::None);
        assert_eq!(ControlAction::Play, ControlAction::Play);
        assert_eq!(ControlAction::Pause, ControlAction::Pause);
        assert_eq!(ControlAction::Step, ControlAction::Step);
        assert_eq!(ControlAction::Reset, ControlAction::Reset);
    }

    #[test]
    fn test_generation_counter() {
        let mut control_panel = ControlPanel::new(42);
        assert_eq!(control_panel.generation, 42);
        
        control_panel.generation = 100;
        assert_eq!(control_panel.generation, 100);
    }
}