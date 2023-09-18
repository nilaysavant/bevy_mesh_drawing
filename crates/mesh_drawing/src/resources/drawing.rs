use bevy::prelude::Entity;
use mesh_geometry_utils::data_structures::MeshPolygon;

/// State of the drawing.
#[derive(Debug, Default, Clone)]
pub struct DrawingState {
    /// Mod of drawing.
    pub mode: DrawingMode,
}

/// Mode of the drawing.
#[derive(Debug, Clone)]
pub enum DrawingMode {
    /// When editing meshes.
    EditMode(EditModeState),
    /// When creating meshes.
    CreateMode(CreateModeState),
}

impl Default for DrawingMode {
    fn default() -> Self {
        Self::EditMode(EditModeState::default())
    }
}

/// Edit mode drawing state.
#[derive(Debug, Default, Clone)]
pub struct EditModeState {
    /// Currently active/selected mesh being manipulated.
    pub active_mesh: Option<Entity>,
    /// Currently active indicator or None if no active.
    pub active_vertex_indicator: Option<Entity>,
}

/// Create mode drawing state.
#[derive(Debug, Default, Clone)]
pub struct CreateModeState {
    /// Currently active/selected mesh ds being manipulated.
    pub mesh_polygon: MeshPolygon,
    /// Check if hovering over draw canvas. Not Yet Implemented.
    pub is_hovering_on_canvas: bool,
}
