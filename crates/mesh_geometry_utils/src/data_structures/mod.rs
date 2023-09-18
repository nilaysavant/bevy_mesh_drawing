/// Module for **Edges** data structure.
pub mod edges;
/// Module for **MeshPolygon** data structure.
pub mod mesh_polygon;
/// Module for the **Ordered SlotMap** data structure.
pub mod ordered_sm;
/// Module for **Vertices** data structure.
pub mod vertices;

pub use edges::{Edge, Edges};
pub use mesh_polygon::MeshPolygon;
pub use ordered_sm::OrderedSlotMap;
pub use vertices::{VertexId, Vertices};
