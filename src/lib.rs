//! ## This Crate
//!
//! The `bevy_mesh_drawing` crate is just a container crate that makes it easier to consume the sub-crates:
//!
//! - `mesh_drawing`: Main Bevy Plugin that allows drawing of meshes.
//! - `mesh_geometry_utils`: Internal crate which provides geometry utils that `mesh_drawing` crate depends on.

// Just re-export the main mesh_drawing plugin...
pub use mesh_drawing::*;
