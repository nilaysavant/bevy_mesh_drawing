use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSystem};
use iyes_loopless::prelude::*;

use crate::{
    events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent},
    prelude::PluginSettings,
    resources::{DrawingMode, DrawingState, PluginState},
    systems::{
        cleanup::cleanup_all,
        create_mode::handle_create_mode_events,
        debug::debug_edit_mode_events,
        drawing_mode::handle_drawing_mode_transition,
        edit_mode::{handle_active_indicator, handle_edit_mode_events},
        grab_transformer::handle_vertex_indicator_grab,
        picker::handle_picker_events,
        raycast::{
            disable_raycast_on_camera_remove, disable_raycast_on_canvas_remove,
            enable_raycast_on_camera_add, enable_raycast_on_canvas_add,
            enable_raycast_on_vertex_indicators_add, handle_raycast_intersections,
            handle_vertex_grabbing_raycast_intersections, setup_raycast,
            update_raycast_with_cursor, MeshDrawingRaycastSet, VertexGrabbingRaycastSet,
        },
        state::initialize_plugin_if_ready,
    },
};

/// # Mesh Drawing Plugin
///
/// Plugin to draw meshes.
pub struct MeshDrawingPlugin;

impl Plugin for MeshDrawingPlugin {
    fn build(&self, app: &mut App) {
        app // plugin app
            // Set default/init plugin state
            .add_loopless_state(PluginState::default())
            // Plugin settings
            .insert_resource(PluginSettings::default())
            .add_plugins(DefaultPickingPlugins)
            // Drawing state
            .insert_resource(DrawingState::default())
            // Configure events...
            .add_event::<EditModeEvent>()
            .add_event::<CreateModeEvent>()
            // Ray-cast stuff...
            .add_plugin(DefaultRaycastingPlugin::<MeshDrawingRaycastSet>::default())
            .add_plugin(DefaultRaycastingPlugin::<VertexGrabbingRaycastSet>::default())
            // Setup updating ray-cast with cursor. Needs to run first.
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_with_cursor
                    .run_in_state(PluginState::Initialized)
                    .before(RaycastSystem::BuildRays::<MeshDrawingRaycastSet>)
                    .before(RaycastSystem::BuildRays::<VertexGrabbingRaycastSet>),
            )
            .add_system(enable_raycast_on_canvas_add)
            .add_system(disable_raycast_on_canvas_remove)
            .add_system(enable_raycast_on_vertex_indicators_add.run_if(is_running_in_edit_mode))
            .add_system(enable_raycast_on_camera_add)
            .add_system(disable_raycast_on_camera_remove)
            .add_startup_system(setup_raycast)
            .add_system(
                handle_raycast_intersections
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_create_mode),
            )
            .add_system(
                handle_vertex_grabbing_raycast_intersections
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_edit_mode),
            )
            // grab transformer
            .add_system(
                handle_vertex_indicator_grab
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_edit_mode),
            )
            // State transition
            .add_system(initialize_plugin_if_ready)
            // Picker stuff...
            .add_system_to_stage(
                CoreStage::PostUpdate,
                handle_picker_events.run_in_state(PluginState::Initialized),
            )
            // edit mode stuff..
            .add_system_to_stage(
                CoreStage::First,
                handle_edit_mode_events
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_edit_mode),
            )
            .add_system(
                handle_active_indicator
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_edit_mode),
            )
            // create mode stuff...
            .add_system_to_stage(
                CoreStage::First,
                handle_create_mode_events
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_create_mode),
            )
            // drawing mode transition...
            .add_system(handle_drawing_mode_transition.run_in_state(PluginState::Initialized))
            // cleanup stuff...
            .add_system_to_stage(CoreStage::Last, cleanup_all)
            // debug stuff...
            .add_system_to_stage(
                CoreStage::First,
                debug_edit_mode_events
                    .run_in_state(PluginState::Initialized)
                    .run_if(is_running_in_edit_mode),
            );
    }
}

fn is_running_in_edit_mode(drawing_state: Res<DrawingState>) -> bool {
    match drawing_state.mode {
        DrawingMode::EditMode(_) => true,
        DrawingMode::CreateMode(_) => false,
    }
}

fn is_running_in_create_mode(drawing_state: Res<DrawingState>) -> bool {
    match drawing_state.mode {
        DrawingMode::EditMode(_) => false,
        DrawingMode::CreateMode(_) => true,
    }
}
