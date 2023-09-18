pub mod components;
pub mod events;
pub mod plugin;
pub mod resources;
pub mod systems;
pub mod utils;

/// Easy access to commonly used modules.
pub mod prelude {
    pub use crate::components::{Canvas, MeshDrawingCamera, PolygonalMesh};
    pub use crate::plugin::MeshDrawingPlugin;
    pub use crate::resources::PluginSettings;
}
