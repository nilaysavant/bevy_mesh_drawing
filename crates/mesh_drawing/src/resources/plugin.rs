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
#[derive(Debug, Clone, Resource)]
pub struct PluginSettings {
    /// Enable insert vertex functionality in edit mode.
    pub is_edit_mode_insert_vertex_enabled: bool,
    /// Enable remove vertex functionality in edit mode.
    pub is_edit_mode_remove_vertex_enabled: bool,
}

impl Default for PluginSettings {
    fn default() -> Self {
        Self {
            is_edit_mode_insert_vertex_enabled: true,
            is_edit_mode_remove_vertex_enabled: true,
        }
    }
}
