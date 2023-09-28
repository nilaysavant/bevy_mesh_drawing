# Bevy Mesh Drawing

**[Experimental ⚠️]** [Bevy][Bevy Engine Website] Plugin to draw polygonal meshes.

![Simple Demo 1](./.readme/simple_demo_1.gif)

## Quick Start

```toml
[dependencies]
bevy_mesh_drawing = { git = "https://github.com/nilaysavant/bevy_mesh_drawing", tag = "v0.2.0" }
```

```rust
/// Add canvas and camera...
fn setup(mut commands: Commands) {
    commands.spawn((
            Name::new("Ground Canvas"),
            PbrBundle { .. },
            Canvas, // Canvas Marker.
        ));
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle { .. },
        MeshDrawingCamera, // Camera Marker.
    ));
}

/// Meshes will be created with [`PolygonalMesh`]
fn handle_polygonal_mesh_add(query: Query<Entity, Added<PolygonalMesh>>) {
    for entity in query.iter() {
        // Use the created mesh here...
    }
}
```

Checkout [`simple.rs`](./examples/simple.rs) in [examples](./examples/).

## Versions

| bevy | bevy_mesh_drawing |
| ---- | ----------------- |
| 0.11 | 0.2.0             |
| 0.8  | 0.1.0             |

[Bevy Engine Website]: https://bevyengine.org/
