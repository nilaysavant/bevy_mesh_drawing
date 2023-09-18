/// State of the plugin.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum PluginState {
    /// Before init.
    UnInitialized,
    /// After init.
    Initialized,
}

impl Default for PluginState {
    fn default() -> Self {
        Self::UnInitialized
    }
}

/// Settings to configure the plugin.
#[derive(Debug, Clone)]
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
