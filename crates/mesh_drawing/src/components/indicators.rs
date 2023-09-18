use bevy::prelude::*;
use mesh_geometry_utils::data_structures::{Edge, VertexId};

/// Polygonal Mesh Indicators component.
///
/// Component to store all the active indicator entities(ids) on the Polygonal Mesh(s).
///
#[derive(Debug, Component)]
pub struct PolygonalMeshIndicators {
    /// Active edges indicator entities on this polygonal mesh.
    pub edges: Vec<Entity>,
    /// Active vertices indicator entities on this polygonal mesh.
    pub vertices: Vec<Entity>,
}

/// Vertex indicator marker component.
///
/// Holds the vertex id of the vertex this indicator is attached to.
#[derive(Debug, Component)]
pub struct VertexIndicator(pub VertexId);

/// Edge indicator marker component.
///
/// Holds the `Edge` data (`from`/`to` *vertex id* of the vertices this indicator connects to).
#[derive(Debug, Component)]
pub struct EdgeIndicator(pub Edge);
