// For Geo crate we need to extend the recursion limit.
#![recursion_limit = "256"]

/// Data structures use to store, represent and generate mesh geometry.
pub mod data_structures;
/// Mesh building and generation utilities.
pub mod mesh_builder;
