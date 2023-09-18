use bevy::prelude::Vec2;
use delegate::delegate;
use slotmap::new_key_type;

use super::{
    ordered_sm::{OrderedSlotMapEnumerateIter, OrderedSlotMapIterator},
    OrderedSlotMap,
};

new_key_type! {
    /// Vertex Id used internally in the Vertices struct.
    pub struct VertexId;
}

/// Vertex is an alias for Vec2.
pub type Vertex = Vec2;

/// Vertices Data Structure.
///
/// Data structure used to represent vertices.
#[derive(Debug, Clone)]
pub struct Vertices(pub OrderedSlotMap<Vertex, VertexId>);

impl Vertices {
    /// Create a new set of Vertices.
    pub fn new() -> Self {
        Self(OrderedSlotMap::new())
    }

    /// Checks if the inserted vertices are in **ClockWise (CW)** Order.
    ///
    /// ### How it Works?
    ///
    /// Sums the edges values using:
    /// ```md
    /// (x2 - x1) * (y2 + y1)
    /// ```
    ///
    /// if sum is `+ve` it is **CW** else **CCW**.
    ///
    /// Stack Overflow: [How to determine if a list of polygon points are in clockwise order?](https://stackoverflow.com/a/1165943)
    pub fn is_order_cw(&self) -> bool {
        let mut edges_sum = 0.;
        for (&Vec2 { x: x1, y: y1 }, &Vec2 { x: x2, y: y2 }) in self.iter().zip(self.iter().skip(1))
        {
            edges_sum += (x2 - x1) * (y2 + y1);
        }
        // Add the last edge
        if let (Some(&Vec2 { x: x1, y: y1 }), Some(&Vec2 { x: x2, y: y2 })) =
            (self.last(), self.first())
        {
            edges_sum += (x2 - x1) * (y2 + y1);
        }
        edges_sum > 0.
    }

    /// Checks if the inserted vertices are in **Counter ClockWise (CCW)** Order.
    ///
    /// Ref: [`Self::is_order_cw`] for more info.
    pub fn is_order_ccw(&self) -> bool {
        !self.is_order_cw()
    }

    delegate! {
        to self.0 {
            /// Get ordered list of vertex ids.
            pub fn ids(&self) -> &Vec<VertexId>;

            /// Reverse the order of vertices.
            pub fn reverse(&mut self);

            /// Get the total number vertices stored.
            pub fn len(&self) -> usize;

            /// Check if empty.
            pub fn is_empty(&self) -> bool;

            /// Push new vertex at the end.
            ///
            /// Returns the pushed vertex's id.
            pub fn push(&mut self, vertex: Vertex) -> VertexId;

            /// Push new vertices at the end.
            ///
            /// Returns the pushed vertices' ids.
            pub fn push_many(&mut self, vertices_vec: Vec<Vertex>) -> Vec<VertexId>;

            /// Insert new vertex at given index.
            ///
            /// Returns the inserted vertex's id.
            pub fn insert(&mut self, index: usize, vertex: Vertex) -> VertexId;

            /// Insert new vertices(s) at given index.
            ///
            /// Returns the inserted vertices' ids.
            pub fn insert_many(&mut self, index: usize, vertices_vec: Vec<Vertex>) -> Vec<VertexId>;

            /// Remove vertex by given id.
            ///
            /// Returns removed vertex.
            pub fn remove(&mut self, id: VertexId) -> Option<Vertex>;

            /// Remove vertices by given ids.
            ///
            /// Returns removed vertices.
            pub fn remove_many(&mut self, ids: Vec<VertexId>) -> Vec<Vertex>;

            /// Clears all vertices(s) and resets the internal slot-map.
            pub fn clear_with_reset(&mut self) -> usize;

            /// Get vertex by ID.
            pub fn get(&self, id: VertexId) -> Option<&Vertex>;

            /// Get mutable vertex by ID.
            pub fn get_mut(&mut self, id: VertexId) -> Option<&mut Vertex>;

            /// Get all (owned) vertices(s) in order.
            pub fn get_all_owned(&self) -> Vec<Vertex>;

            /// Get first vertex.
            pub fn first(&self) -> Option<&Vertex>;

            /// Get last vertex.
            pub fn last(&self) -> Option<&Vertex>;

            /// Returns iterator to enumerate over
            /// all vertices as (key: VertexId, Vertex) pairs.
            pub fn enumerate(&self) -> OrderedSlotMapEnumerateIter<'_, Vertex, VertexId>;

            /// Returns an iterator over vertices.
            pub fn iter(&self) -> OrderedSlotMapIterator<'_, Vertex, VertexId>;
        }
    }
}

impl Default for Vertices {
    /// Create empty Vertices struct.
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Vertex>> for Vertices {
    /// Convert from vector to Vertices.
    fn from(vertices_vec: Vec<Vertex>) -> Self {
        let mut vertices = Self::new();
        // Add vertices
        vertices.push_many(vertices_vec);
        vertices
    }
}

#[test]
fn test_vertices_cw() {
    let vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let vertices = Vertices::from(vec_of_vec2.clone());

    assert!(vertices.is_order_cw());
}

#[test]
fn test_vertices_ccw() {
    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    vec_of_vec2.reverse();
    let vertices = Vertices::from(vec_of_vec2.clone());

    assert!(vertices.is_order_ccw());
}

#[test]
fn test_vertices_push() {
    let mut vec_of_vec2 = vec![
        Vec2::new(0., 0.),
        Vec2::new(0., 1.),
        Vec2::new(1., 1.),
        Vec2::new(1., 0.),
    ];
    let mut vertices = Vertices::new();
    vertices.push_many(vec_of_vec2.clone());

    assert_eq!(vertices.get_all_owned(), vec_of_vec2);
}
