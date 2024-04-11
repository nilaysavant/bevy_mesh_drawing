use bevy::prelude::*;
use bevy_mod_picking::prelude::{Highlight, HighlightKind, PickableBundle};

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
    let highlight_mat_kind = HighlightKind::<StandardMaterial>::Fixed(material_hdl.clone());
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Sphere::new(0.1).mesh()),
            material: material_hdl.clone(),
            transform: Transform::from_translation(Vec3::new(vertex.x, 0., vertex.y)),
            ..default()
        })
        .insert((
            PickableBundle::default(),
            Highlight::<StandardMaterial> {
                hovered: Some(highlight_mat_kind.clone()),
                pressed: Some(highlight_mat_kind.clone()),
                selected: Some(highlight_mat_kind.clone()),
            },
        ))
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
    let highlight_mat_kind = HighlightKind::<StandardMaterial>::Fixed(material.clone());
    // Spawn the edge
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                unlit: true,
                base_color: Color::WHITE,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(mid_pt.x, 0., mid_pt.y),
                rotation: Quat::from_rotation_y(edge_rot_angle_y),
                scale: Vec3::new(
                    edge_vec.length(),
                    EDGE_INDICATOR_WIDTH,
                    EDGE_INDICATOR_WIDTH,
                ),
            },
            ..default()
        })
        .insert((
            PickableBundle::default(),
            Highlight::<StandardMaterial> {
                hovered: Some(highlight_mat_kind.clone()),
                pressed: Some(highlight_mat_kind.clone()),
                selected: Some(highlight_mat_kind.clone()),
            },
        ))
        .id()
}
