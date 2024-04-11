use bevy::prelude::*;

use crate::{
    components::{Canvas, MeshDrawingCamera},
    resources::PluginState,
};

pub fn initialize_plugin_if_ready(
    state: Res<State<PluginState>>,
    mut next_state: ResMut<NextState<PluginState>>,
    query_canvas: Query<(), With<Canvas>>,
    query_camera: Query<(), With<MeshDrawingCamera>>,
) {
    if *state.get() == PluginState::UnInitialized
        && !query_canvas.is_empty()
        && !query_camera.is_empty()
    {
        info!("Plugin ready! Initializing...");
        next_state.set(PluginState::Initialized);
    }
}
