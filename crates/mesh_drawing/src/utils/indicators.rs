use bevy::prelude::*;
use bevy_mod_picking::prelude::{Highlight, PickableBundle};

/// Spawns a vertex indicator at the given vertex.
pub fn spawn_vertex_indicators(
    vertex: Vec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let material = StandardMaterial {
        unlit: true,
        base_color: Color::WHITE,
        ..default()
    };
    let material_hdl = materials.add(material);
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(
                Mesh::try_from(shape::Icosphere {
                    radius: 0.1,
                    ..default()
                })
                .unwrap(),
            ),
            material: material_hdl.clone(),
            transform: Transform::from_translation(Vec3::new(vertex.x, 0., vertex.y)),
            ..default()
        })
        .insert(PickableBundle::default())
        .id()
}

/// Width of the edge indicator.
pub const EDGE_INDICATOR_WIDTH: f32 = 0.1;

/// Spawns a new edge indicator between the `from` and `to` vertex (`VertexId`).
pub fn spawn_edge_indicator(
    from_vert: Vec2,
    to_vert: Vec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let mid_pt = (to_vert + from_vert) / 2.;
    let edge_vec = to_vert - from_vert;
    let edge_rot_angle_y = edge_vec.angle_between(Vec2::X);
    // create material for edge
    let material = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::WHITE,
        ..default()
    });
    // Spawn the edge
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                unlit: true,
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(mid_pt.x, 0., mid_pt.y),
                rotation: Quat::from_rotation_y(edge_rot_angle_y),
                scale: Vec3::new(
                    edge_vec.length() / 2.,
                    EDGE_INDICATOR_WIDTH,
                    EDGE_INDICATOR_WIDTH,
                ),
            },
            ..default()
        })
        .insert(PickableBundle::default())
        .id()
}
