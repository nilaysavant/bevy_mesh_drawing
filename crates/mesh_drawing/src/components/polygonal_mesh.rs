use bevy::prelude::*;
use mesh_geometry_utils::data_structures::MeshPolygon;

/// Polygonal Mesh component.
///
/// Use to mark meshes created using the `MeshPolygon` data struct.
///
/// Holds the `MeshPolygon` data used to construct this mesh.
#[derive(Debug, Component, Default)]
pub struct PolygonalMesh {
    /// The polygon used to extrude into this mesh.
    pub mesh_polygon: MeshPolygon,
    /// Handle to the current mesh of the editing entity.
    pub mesh_handle: Option<Handle<Mesh>>,
}
