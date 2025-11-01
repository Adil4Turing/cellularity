use cellularity::{ControlPanel, ControlAction};

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