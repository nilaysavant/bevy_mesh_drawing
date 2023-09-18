use bevy::utils::{hashbrown::hash_set::Iter, HashSet};
use delegate::delegate;

use super::VertexId;

/// Edge of a polygon.
///
/// Connects vertex with `from<id>` to vertex with `to<id>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    pub from: VertexId,
    pub to: VertexId,
}

impl Edge {
    /// Create a new edge from given vertex ids: `from<id>` and `to<id>`.
    pub fn new(from: VertexId, to: VertexId) -> Self {
        Self { from, to }
    }
}

/// Edges Data Structure
///
/// Used to represent edges of a polygon.
#[derive(Debug, Clone)]
pub struct Edges(pub HashSet<Edge>);

impl Edges {
    /// Create a new set of Edges.
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    /// Get all (owned) edge(s).
    pub fn get_all_owned(&self) -> Vec<Edge> {
        let mut edges_vec = vec![];
        for edge in self.values() {
            edges_vec.push(edge.clone());
        }
        edges_vec
    }

    /// Filter edges that contain the vertex id.
    pub fn filter_by_vertex(&self, vertex_id: VertexId) -> impl Iterator<Item = &Edge> {
        self.iter()
            .filter(move |e| e.from == vertex_id || e.to == vertex_id)
    }

    delegate! {
        to self.0 {
            /// Returns iterator over values of the stored edges.
            #[call(iter)]
            pub fn values(&self) -> Iter<Edge>;
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
            pub fn clear(&mut self);
            /// Checks if contains the edge.
            pub fn contains(&self, edge: &Edge) -> bool;
            pub fn get(&self, edge: &Edge) -> Option<&Edge>;
            pub fn take(&mut self, edge: &Edge) -> Option<Edge>;
            /// Insert new edge to the Edges.
            ///
            /// If did not have this edge present, true is returned.
            ///
            /// If did have this edge present, false is returned.
            pub fn insert(&mut self, edge: Edge) -> bool;
            pub fn remove(&mut self, edge: &Edge) -> bool;
            pub fn iter(&self) -> Iter<Edge>;
        }
    }
}

impl Default for Edges {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Edge>> for Edges {
    fn from(edges_vec: Vec<Edge>) -> Self {
        let mut edges = Edges::new();
        for edge in edges_vec {
            edges.insert(edge);
        }
        edges
    }
}

#[test]
fn test_edges_basic() {
    use super::Vertices;
    use bevy::prelude::Vec2;

    let vertices = Vertices::from(vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ]);
    let mut edges_vec = vec![];
    for (v1, v2) in vertices.ids().iter().zip(vertices.ids().iter().skip(1)) {
        edges_vec.push(Edge::new(*v1, *v2));
    }
    let edges_vec_iter = edges_vec.iter();
    let mut edges = Edges::from(edges_vec_iter.clone().take(2).cloned().collect::<Vec<_>>());

    for edge in edges_vec_iter.skip(2) {
        edges.insert(edge.clone());
    }

    for edge in edges_vec.iter() {
        assert!(edges.contains(edge));
    }
}
