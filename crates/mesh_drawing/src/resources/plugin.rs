use bevy::prelude::*;

/// State of the plugin.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, States)]
pub enum PluginState {
    /// Before init.
    #[default]
    UnInitialized,
    /// After init.
    Initialized,
}

/// Settings to configure the plugin.
#[derive(Debug, Clone, Copy, Resource)]
pub struct MeshDrawingPluginSettings {
    /// Size/height of the extruded [`Mesh`] from [`MeshPolygon`](mesh_geometry_utils::data_structures::MeshPolygon)
    pub extrude_size: f32,
    /// Enable insert vertex functionality in edit mode.
    pub is_edit_mode_insert_vertex_enabled: bool,
    /// Enable remove vertex functionality in edit mode.
    pub is_edit_mode_remove_vertex_enabled: bool,
    /// Input bindings for the plugin.
    pub input_binds: MeshDrawingPluginInputBinds,
}

impl Default for MeshDrawingPluginSettings {
    fn default() -> Self {
        Self {
            extrude_size: 2.0,
            is_edit_mode_insert_vertex_enabled: true,
            is_edit_mode_remove_vertex_enabled: true,
            input_binds: MeshDrawingPluginInputBinds::default(),
        }
    }
}

/// Input/Key binds for the plugin.
#[derive(Debug, Clone, Copy, Resource)]
pub struct MeshDrawingPluginInputBinds {
    /// [`KeyCode`] used to switch to [`EditMode`](`super::DrawingMode::EditMode`)
    pub edit_mode_switch_key: KeyCode,
    /// [`KeyCode`] to switch to [`CreateMode`](`super::DrawingMode::CreateMode`)
    pub create_mode_switch_key: KeyCode,
    /// [`KeyCode`] used to remove existing vertex.
    ///
    /// Remove happens on this `KeyDown` + `LMB Click` on the desired Vertex.
    pub edit_mode_remove_vertex_key: KeyCode,
    /// [`KeyCode`] used to insert new vertex on Edge.
    ///
    /// Vertex is inserted when this `KeyDown` + `LMB Click` on the desired Edge.
    pub edit_mode_insert_vertex_key: KeyCode,
    /// [`MouseButton`] input used to _add vertex_ in [`CreateMode`](`super::DrawingMode::CreateMode`)
    pub create_mode_add_vertex_btn: MouseButton,
    /// [`MouseButton`] input used to _close polygon and create mesh_ in [`CreateMode`](`super::DrawingMode::CreateMode`)
    pub create_mode_close_and_extrude_mesh_btn: MouseButton,
}

impl Default for MeshDrawingPluginInputBinds {
    fn default() -> Self {
        Self {
            edit_mode_switch_key: KeyCode::Digit1,
            create_mode_switch_key: KeyCode::Digit2,
            edit_mode_remove_vertex_key: KeyCode::AltLeft,
            edit_mode_insert_vertex_key: KeyCode::ControlLeft,
            create_mode_add_vertex_btn: MouseButton::Left,
            create_mode_close_and_extrude_mesh_btn: MouseButton::Right,
        }
    }
}
