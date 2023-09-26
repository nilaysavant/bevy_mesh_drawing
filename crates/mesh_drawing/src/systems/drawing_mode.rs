use bevy::prelude::*;

use crate::{
    events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent},
    resources::MeshDrawingPluginSettings,
};

pub fn handle_drawing_mode_transition(
    keyboard_input: Res<Input<KeyCode>>,
    settings: Res<MeshDrawingPluginSettings>,
    mut edit_mode_event: EventWriter<EditModeEvent>,
    mut create_mode_event: EventWriter<CreateModeEvent>,
) {
    let MeshDrawingPluginSettings { input_binds, .. } = *settings;
    if keyboard_input.just_pressed(input_binds.edit_mode_switch_key) {
        info!("Change to edit mode...");
        create_mode_event.send(CreateModeEvent::EditModeSwitch);
    } else if keyboard_input.just_pressed(input_binds.create_mode_switch_key) {
        info!("Change to create mode...");
        edit_mode_event.send(EditModeEvent::CreateModeSwitch);
    }
}
