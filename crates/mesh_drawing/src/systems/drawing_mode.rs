use bevy::prelude::*;

use crate::{
    events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent},
    prelude::PluginSettings,
};

pub fn handle_drawing_mode_transition(
    keyboard_input: Res<Input<KeyCode>>,
    settings: Res<PluginSettings>,
    mut edit_mode_event: EventWriter<EditModeEvent>,
    mut create_mode_event: EventWriter<CreateModeEvent>,
) {
    let PluginSettings { key_binds, .. } = *settings;
    if keyboard_input.just_pressed(key_binds.edit_mode_switch_key) {
        info!("Change to edit mode...");
        create_mode_event.send(CreateModeEvent::EditModeSwitch);
    } else if keyboard_input.just_pressed(key_binds.create_mode_switch_key) {
        info!("Change to create mode...");
        edit_mode_event.send(EditModeEvent::CreateModeSwitch);
    }
}
