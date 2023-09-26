pub mod components;
pub mod events;
pub mod plugin;
pub mod resources;
pub mod systems;
pub mod utils;

/// Easy access to commonly used modules.
pub mod prelude {
    // plugin...
    pub use crate::plugin::MeshDrawingPlugin;
    // components...
    pub use crate::components::{Canvas, MeshDrawingCamera, PolygonalMesh};
    // settings...
    pub use crate::resources::{MeshDrawingPluginKeyBinds, MeshDrawingPluginSettings};
}
