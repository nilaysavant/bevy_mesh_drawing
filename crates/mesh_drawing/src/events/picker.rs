use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct PickerClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PickerClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        PickerClickEvent(event.target, event.hit.depth)
    }
}