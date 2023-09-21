use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastSystem};

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
            // Plugin settings
            .insert_resource(PluginSettings::default())
            .add_plugins(DefaultPickingPlugins)
            // Drawing state
            .insert_resource(DrawingState::default())
            // Configure events...
            .add_event::<EditModeEvent>()
            .add_event::<CreateModeEvent>()
            // Ray-cast stuff...
            .add_plugins(DefaultRaycastingPlugin::<MeshDrawingRaycastSet>::default())
            .add_plugins(DefaultRaycastingPlugin::<VertexGrabbingRaycastSet>::default())
            // add state
            .add_state::<PluginState>()
            // Setup updating ray-cast with cursor. Needs to run first.
            .add_systems(
                First,
                update_raycast_with_cursor
                    .run_if(in_state(PluginState::Initialized))
                    .before(RaycastSystem::BuildRays::<MeshDrawingRaycastSet>)
                    .before(RaycastSystem::BuildRays::<VertexGrabbingRaycastSet>),
            )
            .add_systems(Update, enable_raycast_on_canvas_add)
            .add_systems(Update, disable_raycast_on_canvas_remove)
            .add_systems(
                Update,
                enable_raycast_on_vertex_indicators_add.run_if(is_running_in_edit_mode),
            )
            .add_systems(Update, enable_raycast_on_camera_add)
            .add_systems(Update, disable_raycast_on_camera_remove)
            .add_systems(Startup, setup_raycast)
            .add_systems(
                Update,
                handle_raycast_intersections
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_create_mode),
            )
            .add_systems(
                Update,
                handle_vertex_grabbing_raycast_intersections
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_edit_mode),
            )
            // grab transformer
            .add_systems(
                Update,
                handle_vertex_indicator_grab
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_edit_mode),
            )
            // State transition
            .add_systems(Update, initialize_plugin_if_ready)
            // Picker stuff...
            .add_systems(
                PostUpdate,
                handle_picker_events.run_if(in_state(PluginState::Initialized)),
            )
            // edit mode stuff..
            .add_systems(
                First,
                handle_edit_mode_events
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_edit_mode),
            )
            .add_systems(
                Update,
                handle_active_indicator
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_edit_mode),
            )
            // create mode stuff...
            .add_systems(
                First,
                handle_create_mode_events
                    .run_if(in_state(PluginState::Initialized))
                    .run_if(is_running_in_create_mode),
            )
            // drawing mode transition...
            .add_systems(
                Update,
                handle_drawing_mode_transition.run_if(in_state(PluginState::Initialized)),
            )
            // cleanup stuff...
            .add_systems(Last, cleanup_all)
            // debug stuff...
            .add_systems(
                First,
                debug_edit_mode_events
                    .run_if(in_state(PluginState::Initialized))
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
