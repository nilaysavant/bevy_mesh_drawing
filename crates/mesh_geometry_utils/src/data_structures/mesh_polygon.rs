use bevy::prelude::Mesh;

use crate::mesh_builder::earcutr::generate_mesh_earcutr;

use super::{vertices::Vertex, Edge, Edges, VertexId, Vertices};

/// # Mesh Polygon Data Structure
///
/// A polygon data structure used to create mesh.
#[derive(Debug, Clone)]
pub struct MeshPolygon {
    /// Vertices of the polygon.
    pub vertices: Vertices,
    /// Edges of the polygon.
    pub edges: Edges,
}

impl MeshPolygon {
    /// Create a new mesh polygon.
    pub fn new() -> Self {
        Self {
            vertices: Vertices::new(),
            edges: Edges::new(),
        }
    }

    /// Clear and reset the polygon.
    pub fn clear_with_reset(&mut self) {
        self.vertices.clear_with_reset();
        self.edges.clear();
    }

    /// Reverse the order of vertices and edges in polygon
    pub fn reverse(&mut self) {
        // reverse the vertices ids
        self.vertices.reverse();
        let mut updated_edges = vec![];
        // collect list of reversed edges
        for Edge { from, to } in self.edges.iter() {
            updated_edges.push(Edge::new(*to, *from));
        }
        // clear and insert the reversed edges...
        self.edges.clear();
        for edge in updated_edges {
            self.edges.insert(edge);
        }
    }

    /// Push a new vertex to the end and return its Id.
    ///
    /// Also adds an edge if more than 2 vertices are present.
    pub fn push_vertex(&mut self, vertex: Vertex) -> VertexId {
        let id = self.vertices.push(vertex);
        let vertices_len = self.vertices.len();
        if vertices_len > 1 {
            if let Some(prev_vertex) = self.vertices.ids().get(vertices_len - 2) {
                self.edges.insert(Edge::new(*prev_vertex, id));
            }
        }
        id
    }

    /// Close the polygon connecting end back to start.
    ///
    /// Only possible when more than 2 vertices are present.
    ///
    /// Returns if close was success.
    pub fn close(&mut self) -> bool {
        if self.vertices.len() > 2 {
            let vertex_ids = self.vertices.ids();
            if let (Some(last), Some(first)) = (vertex_ids.last(), vertex_ids.first()) {
                self.edges.insert(Edge::new(*last, *first));
                return true;
            }
        }
        false
    }

    /// Extrude the polygon into a Bevy Mesh.
    ///
    /// Internally tries to close the polygon
    /// before generating the mesh using the mesh builder.
    pub fn extrude_to_bevy_mesh(&mut self, extrude_size: f32) -> Option<Mesh> {
        if self.close() {
            let mesh = generate_mesh_earcutr(self.vertices.get_all_owned(), extrude_size);
            return Some(mesh);
        }
        None
    }

    /// Remove vertex by Id.
    ///
    /// Removes any edges connecting to it.
    ///
    /// Adds new edge that connects the neighboring vertices.
    ///
    /// Returns:
    /// - `Option<Vertex>` : Removed vertex or `None` if not removed.
    /// - `Vec<Edge>` : Any added edges.
    pub fn remove_vertex(&mut self, vertex_id: VertexId) -> (Option<Vertex>, Vec<Edge>) {
        let vertex = self.vertices.remove(vertex_id);
        let orphan_edges = self
            .edges
            .iter()
            .filter(|e| e.from == vertex_id || e.to == vertex_id)
            .cloned()
            .collect::<Vec<_>>();
        let mut new_edge_from = None;
        let mut new_edge_to = None;
        for edge in orphan_edges {
            if edge.to == vertex_id {
                new_edge_from = Some(edge.from);
            } else if edge.from == vertex_id {
                new_edge_to = Some(edge.to);
            }
            // cleanup orphan edges
            self.edges.remove(&edge);
        }
        let mut added_edges = vec![];
        if let (Some(from), Some(to)) = (new_edge_from, new_edge_to) {
            // Add the edge connecting the neighboring vertices.
            let edge = Edge::new(from, to);
            self.edges.insert(edge.clone());
            added_edges.push(edge);
        }
        (vertex, added_edges)
    }

    /// Inserts a new vertex on the specified edge.
    ///
    /// Also adds the connecting edges and removed existing edge.
    ///
    /// Returns the inserted vertex id or None if not inserted.
    pub fn insert_vertex_on_edge(&mut self, vertex: Vertex, edge: Edge) -> Option<VertexId> {
        let Edge { from, to } = edge;
        // get the from/to idx.
        let Some(from_idx) = self.vertices.ids().iter().position(|id| id == &from) else {
            return None;
        };
        let Some(to_idx) = self.vertices.ids().iter().position(|id| id == &to) else {
            return None;
        };
        // Get correct idx as order could be reversed
        let length = self.vertices.len();
        let insert_idx =
            if (from_idx == 0 && to_idx == length - 1) || (from_idx == length - 1 && to_idx == 0) {
                // if last edge, insert at length
                length
            } else if from_idx < to_idx {
                from_idx + 1
            } else {
                to_idx + 1
            };
        // insert new vert at from + 1
        let id = self.vertices.insert(insert_idx, vertex);
        // remove edge
        self.edges.remove(&edge);
        // create new connecting edges
        self.edges.insert(Edge { from, to: id });
        self.edges.insert(Edge { from: id, to });
        Some(id)
    }
}

impl Default for MeshPolygon {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Vertex>> for MeshPolygon {
    /// Convert from vector of vertices to polygon.
    fn from(vertices_vec: Vec<Vertex>) -> Self {
        let mut polygon = Self::new();
        for vertex in vertices_vec {
            polygon.push_vertex(vertex);
        }
        // close if possible
        polygon.close();
        polygon
    }
}

#[test]
fn test_basic_mesh_polygon() {
    use bevy::prelude::Vec2;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    for vertex in vec_of_vec2.iter() {
        polygon.push_vertex(*vertex);
    }
    assert_eq!(polygon.vertices.len(), vec_of_vec2.len());
    assert_eq!(polygon.edges.len(), vec_of_vec2.len() - 1);
    polygon.close();
    assert_eq!(polygon.edges.len(), vec_of_vec2.len());
}

#[test]
fn test_mesh_polygon_from_vec_vertices() {
    use bevy::prelude::Vec2;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::from(vec_of_vec2.clone());

    assert_eq!(polygon.vertices.len(), vec_of_vec2.len());
    assert_eq!(polygon.edges.len(), vec_of_vec2.len());
}

#[test]
fn test_basic_edges() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push vertices of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // assert correct vertices.
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, b));
    expected_edges.insert(Edge::new(b, c));
    expected_edges.insert(Edge::new(c, d));
    expected_edges.insert(Edge::new(d, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_basic_edges_with_vertices_reversed() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push vertices of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // reverse order of vertices
    polygon.reverse();
    // assert correct vertices...
    vec_of_vec2.reverse();
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, d));
    expected_edges.insert(Edge::new(d, c));
    expected_edges.insert(Edge::new(c, b));
    expected_edges.insert(Edge::new(b, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_b() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(b);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![a, c, d]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, c));
    expected_edges.insert(Edge::new(c, d));
    expected_edges.insert(Edge::new(d, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_b_reversed_vertices() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // reverse
    polygon.reverse();
    vec_of_vec2.reverse();
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(b);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![d, c, a]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, d));
    expected_edges.insert(Edge::new(d, c));
    expected_edges.insert(Edge::new(c, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_d() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(d);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![a, b, c]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, b));
    expected_edges.insert(Edge::new(b, c));
    expected_edges.insert(Edge::new(c, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_d_reversed_vertices() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // reverse
    polygon.reverse();
    vec_of_vec2.reverse();
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(d);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![c, b, a]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(a, c));
    expected_edges.insert(Edge::new(c, b));
    expected_edges.insert(Edge::new(b, a));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_a() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(a);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![b, c, d]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(b, c));
    expected_edges.insert(Edge::new(c, d));
    expected_edges.insert(Edge::new(d, b));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}

#[test]
fn test_vertex_remove_a_reversed_vertices() {
    use bevy::prelude::Vec2;
    use bevy::utils::HashSet;

    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut polygon = MeshPolygon::new();
    // Push verts of polygon
    let a = polygon.push_vertex(vec_of_vec2[0]);
    let b = polygon.push_vertex(vec_of_vec2[1]);
    let c = polygon.push_vertex(vec_of_vec2[2]);
    let d = polygon.push_vertex(vec_of_vec2[3]);
    // close polygon
    assert!(polygon.close());
    // reverse
    polygon.reverse();
    vec_of_vec2.reverse();
    // assert correct vertices...
    assert_eq!(polygon.vertices.get_all_owned(), vec_of_vec2);
    // remove
    polygon.remove_vertex(a);
    // assert correct vertex ids
    assert_eq!(polygon.vertices.ids().clone(), vec![d, c, b]);
    // calc expected edges...
    let mut expected_edges = HashSet::new();
    expected_edges.insert(Edge::new(b, d));
    expected_edges.insert(Edge::new(d, c));
    expected_edges.insert(Edge::new(c, b));
    // assert correct edges
    assert_eq!(polygon.edges.0, expected_edges);
}
