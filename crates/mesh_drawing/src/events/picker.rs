use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event, Deref, DerefMut)]
pub struct PickerClickEvent(pub ListenerInput<Pointer<Click>>);

impl From<ListenerInput<Pointer<Click>>> for PickerClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        PickerClickEvent(event)
    }
}
