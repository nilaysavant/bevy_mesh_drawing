//! Simple Example
//!
//! KeyBinds are currently hardcoded:
//!
//! `Key1`: For Edit Mode (Allows editing existing meshes created with this plugin)
//! `Key2`: For Create Mode (Allows creating new meshes created with this plugin)
//! `CtrlLeft` + `LMB` Click: [Edit Mode] Insert new vertex on edge
//! `AltLeft` + `LMB` Click: [Edit Mode] Delete existing vertex.

use bevy::prelude::*;
use bevy_mesh_drawing::prelude::{Canvas, MeshDrawingCamera, MeshDrawingPlugin, PolygonalMesh};

pub fn main() {
    App::new() // App
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshDrawingPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_window)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_polygonal_mesh_add)
        .run();
}

/// # Setup Window
///
/// System updates and sets up the window attributes
pub fn setup_window(mut windows: Query<&mut Window>) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    window.title = "simple mesh drawing".to_string();
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
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 20.0,
                ..default()
            })),
            material: materials.add(Color::rgba(0.3, 0.5, 0.3, 1.0).into()),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                // rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ..default()
            },
            ..default()
        })
        .insert(Canvas)
        .insert(Name::new("Ground Canvas"));
    // light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::splat(10.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MeshDrawingCamera)
        .insert(Name::new("Camera"));
}

pub fn handle_polygonal_mesh_add(query: Query<Entity, Added<PolygonalMesh>>) {
    for entity in query.iter() {
        info!("Created polygonal mesh: {:?}", entity);
    }
}
