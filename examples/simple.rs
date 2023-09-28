//! Simple Example
//!
//! Current defaults:
//!
//! `Key1`: For Edit Mode (Allows editing existing meshes created with this plugin)
//! `Key2`: For Create Mode (Allows creating new meshes created with this plugin)
//! `MouseButton::Left` Click on Canvas: [Create Mode] Used to create vertex.
//! `MouseButton::Right` Click on Canvas: [Create Mode] Used to close the polygon and extrude it into a Mesh.
//! `CtrlLeft` + `LMB` Click: [Edit Mode] Insert new vertex on edge
//! `AltLeft` + `LMB` Click: [Edit Mode] Delete existing vertex.

use bevy::prelude::*;
use bevy_mesh_drawing::prelude::{
    Canvas, MeshDrawingCamera, MeshDrawingPlugin, MeshDrawingPluginInputBinds,
    MeshDrawingPluginSettings, PolygonalMesh,
};

pub fn main() {
    App::new() // App
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "simple mesh drawing".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MeshDrawingPlugin)
        .insert_resource(MeshDrawingPluginSettings {
            extrude_size: 2.0, // config extrude height
            // config input binds...
            input_binds: MeshDrawingPluginInputBinds {
                edit_mode_switch_key: KeyCode::Key1, // config key to switch to edit mode
                create_mode_switch_key: KeyCode::Key2, // config key to switch to create mode
                ..default()
            },
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_polygonal_mesh_add)
        .run();
}

/// Setup scene.
///
/// set up a simple 3D scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground canvas
    commands.spawn((
        Name::new("Ground Canvas"),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 20.0,
                ..default()
            })),
            material: materials.add(Color::rgba(0.3, 0.5, 0.3, 1.0).into()),
            ..default()
        },
        Canvas, // Mark this entity to allow drawing on it.
    ));
    // camera
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::splat(10.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MeshDrawingCamera, // Mark camera for use with drawing.
    ));
    // light
    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));
}

/// Drawn meshes will be created with [`PolygonalMesh`] component.
pub fn handle_polygonal_mesh_add(query: Query<Entity, Added<PolygonalMesh>>) {
    for entity in query.iter() {
        // Use the created mesh here...
        info!("Created polygonal mesh: {:?}", entity);
    }
}
