use bevy::prelude::{Entity, Vec3};

/// Create Mode Event enum.
#[derive(Debug, Clone)]
pub enum CreateModeEvent {
    /// Triggered when a vertex needs to be added to active `MeshPolygon`.
    VertexAdd(Vec3),
    /// Triggered when we need to close the polygon. And Create/extrude it into mesh.
    PolygonCloseAndIntoMeshExtrude,
    /// Triggered when drawing mode is switched to `EditMode`.
    EditModeSwitch,
}
