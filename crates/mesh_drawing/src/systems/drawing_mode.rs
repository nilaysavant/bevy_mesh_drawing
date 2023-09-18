use bevy::prelude::*;

use crate::events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent};

pub fn handle_drawing_mode_transition(
    keyboard_input: Res<Input<KeyCode>>,
    mut edit_mode_event: EventWriter<EditModeEvent>,
    mut create_mode_event: EventWriter<CreateModeEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        info!("Change to edit mode...");
        create_mode_event.send(CreateModeEvent::EditModeSwitch);
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        info!("Change to create mode...");
        edit_mode_event.send(EditModeEvent::CreateModeSwitch);
    }
}
