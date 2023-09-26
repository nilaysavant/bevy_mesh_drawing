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
}

impl Default for MeshDrawingPluginKeyBinds {
    fn default() -> Self {
        Self {
            edit_mode_switch_key: KeyCode::Key1,
            create_mode_switch_key: KeyCode::Key2,
        }
    }
}
