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
    /// Key binds for the plugin.
    pub key_binds: MeshDrawingPluginKeyBinds,
}

impl Default for MeshDrawingPluginSettings {
    fn default() -> Self {
        Self {
            extrude_size: 2.0,
            is_edit_mode_insert_vertex_enabled: true,
            is_edit_mode_remove_vertex_enabled: true,
            key_binds: MeshDrawingPluginKeyBinds::default(),
        }
    }
}

/// Key binds for the plugin.
#[derive(Debug, Clone, Copy, Resource)]
pub struct MeshDrawingPluginKeyBinds {
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
}

impl Default for MeshDrawingPluginKeyBinds {
    fn default() -> Self {
        Self {
            edit_mode_switch_key: KeyCode::Key1,
            create_mode_switch_key: KeyCode::Key2,
            edit_mode_remove_vertex_key: KeyCode::AltLeft,
            edit_mode_insert_vertex_key: KeyCode::ControlLeft,
        }
    }
}
