use bevy::prelude::*;
use iyes_loopless::state::{CurrentState, NextState};

use crate::{
    components::{Canvas, MeshDrawingCamera},
    resources::PluginState,
};

pub fn initialize_plugin_if_ready(
    mut commands: Commands,
    state: Res<CurrentState<PluginState>>,
    query_canvas: Query<With<Canvas>>,
    query_camera: Query<With<MeshDrawingCamera>>,
) {
    if state.0 == PluginState::UnInitialized && !query_canvas.is_empty() && !query_camera.is_empty()
    {
        info!("Plugin ready! Initializing...");
        commands.insert_resource(NextState(PluginState::Initialized));
    }
}
