use bevy::prelude::{Entity, Vec3, Event};
use mesh_geometry_utils::data_structures::Edge;

/// Edit Mode Event enum.
#[derive(Debug, Clone, Event)]
pub enum EditModeEvent {
    /// Triggered when a Mesh entity is selected.
    ///
    /// `Entity`: Selected polygonal mesh entity.
    PolygonalMeshSelect(Entity),
    /// Triggered when a vertex indicator is just pressed.
    ///
    /// `Entity`: Vertex indicator entity.
    VertexIndicatorJustPressed(Entity),
    /// Triggered when a vertex indicator is just released.
    VertexIndicatorJustReleased,
    /// Reset. When all entity/indicators need to be deselected & deactivated.
    Reset,
    /// Triggered when drawing mode is switched to `CreateMode`.
    CreateModeSwitch,
    /// Triggered when a new vertex needs to be inserted at an existing edge.
    VertexInsert(InsertVertexData),
    /// Triggered when an existing vertex needs to be deleted/removed.
    ///
    /// `Entity`: Vertex indicator entity.
    VertexRemove(Entity),
}

/// Data of the `VertexInsert` event.
#[derive(Debug, Clone)]
pub struct InsertVertexData {
    /// Edge on which new vertex needs to be inserted.
    pub edge: Edge,
    /// Translation/position of insertion.
    pub translation: Vec3,
}
