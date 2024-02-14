use bevy::prelude::*;

use crate::events::edit_mode::EditModeEvent;

pub fn debug_edit_mode_events(mut events: EventReader<EditModeEvent>) {
    for event in events.read() {
        info!("EditModeEvent: {:?}", event);
    }
}
