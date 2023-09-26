use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_mod_picking::prelude::{Highlight, HighlightKind, PickableBundle};
use mesh_geometry_utils::data_structures::Edge;

use crate::{
    components::{Canvas, Cleanup, EdgeIndicator, PolygonalMesh, VertexIndicator},
    events::create_mode::CreateModeEvent,
    resources::MeshDrawingPluginSettings,
    resources::{drawing::EditModeState, DrawingMode, DrawingState},
    utils::{
        canvas_correction::get_canvas_corrected_translation,
        indicators::{spawn_edge_indicator, spawn_vertex_indicators},
    },
};

/// Squared dist below which vertex is merged.
const MERGE_BELOW_DIST_SQUARED: f32 = 0.1;

/// Handle create mode events.
#[allow(clippy::too_many_arguments)]
pub fn handle_create_mode_events(
    mut commands: Commands,
    query_canvas: Query<(Entity, &Transform), With<Canvas>>,
    query_indicators: Query<Entity, (With<VertexIndicator>, Without<Cleanup>)>,
    query_edge_indicators: Query<Entity, (With<EdgeIndicator>, Without<Cleanup>)>,
    mut events: EventReader<CreateModeEvent>,
    settings: Res<MeshDrawingPluginSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut drawing_state: ResMut<DrawingState>,
) {
    let Ok((canvas_entity, canvas_transform)) = query_canvas.get_single() else {
        return;
    };
    for event in events.iter() {
        let DrawingMode::CreateMode(create_mode_state) = &mut drawing_state.mode else {
            return;
        };
        match event {
            CreateModeEvent::VertexAdd(intersection_point) => {
                let intersection_point =
                    get_canvas_corrected_translation(*intersection_point, canvas_transform);
                // Add new vertex
                if let Some(first_vert) = create_mode_state.mesh_polygon.vertices.first() {
                    // check if new intersection overlaps first vertex, if it does then close the polygon
                    // instead of adding new vertex
                    let dist_square_from_first_vert =
                        intersection_point.xz().distance_squared(*first_vert);
                    if dist_square_from_first_vert <= MERGE_BELOW_DIST_SQUARED {
                        if let Err(error) = close_polygon_and_extrude_mesh(
                            settings.extrude_size,
                            create_mode_state,
                            &mut meshes,
                            &mut materials,
                            &mut commands,
                            canvas_entity,
                        ) {
                            error!("error: {:?}", error);
                            return;
                        }
                        cleanup_create_mode_entities(
                            &mut commands,
                            &query_edge_indicators,
                            &query_indicators,
                        );
                        continue;
                    }
                }
                // push new point in vertices
                let vertex_id = create_mode_state
                    .mesh_polygon
                    .push_vertex(intersection_point.xz());
                // Draw vertex indicator
                let entity = spawn_vertex_indicators(
                    intersection_point.xz(),
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
                // Mark as indicator
                commands.entity(entity).insert(VertexIndicator(vertex_id));
                // push vertex indicator as child of canvas
                commands.entity(canvas_entity).add_child(entity);
                // find the new edge inserted (if any) & draw it
                if let Some(Edge { from, to }) = create_mode_state
                    .mesh_polygon
                    .edges
                    .filter_by_vertex(vertex_id)
                    .last()
                    .cloned()
                {
                    if let (Some(from_vert), Some(to_vert)) = (
                        create_mode_state.mesh_polygon.vertices.get(from),
                        create_mode_state.mesh_polygon.vertices.get(to),
                    ) {
                        let entity = spawn_edge_indicator(
                            *from_vert,
                            *to_vert,
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                        );
                        commands
                            .entity(entity)
                            .insert(EdgeIndicator(Edge { from, to }));
                        // push edge indicator as child of canvas
                        commands.entity(canvas_entity).add_child(entity);
                    }
                }
            }
            CreateModeEvent::PolygonCloseAndIntoMeshExtrude => {
                if let Err(error) = close_polygon_and_extrude_mesh(
                    settings.extrude_size,
                    create_mode_state,
                    &mut meshes,
                    &mut materials,
                    &mut commands,
                    canvas_entity,
                ) {
                    error!("error: {:?}", error);
                    continue;
                }
                cleanup_create_mode_entities(
                    &mut commands,
                    &query_edge_indicators,
                    &query_indicators,
                );
            }
            CreateModeEvent::EditModeSwitch => {
                cleanup_create_mode_entities(
                    &mut commands,
                    &query_edge_indicators,
                    &query_indicators,
                );
                // switch to create mode.
                drawing_state.mode = DrawingMode::EditMode(EditModeState::default());
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn close_polygon_and_extrude_mesh(
    extrude_size: f32,
    create_mode_state: &mut crate::resources::drawing::CreateModeState,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    commands: &mut Commands,
    canvas_entity: Entity,
) -> Result<(), String> {
    if create_mode_state.mesh_polygon.vertices.len() < 3 {
        return Err("Vertices are less than 3!".to_string());
    }
    if create_mode_state.mesh_polygon.vertices.is_order_ccw() {
        // order needs to be in cw.
        // Else the side faces are not rendered properly!
        // PS: In bevy it might look asif curve is drawn cw,
        // but internally it results in ccw. Somehow. IT IS OPPOSITE!
        create_mode_state.mesh_polygon.reverse();
    }
    // Create mesh from vertices
    let mut generated_mesh = create_mode_state
        .mesh_polygon
        .extrude_to_bevy_mesh(extrude_size)
        .ok_or_else(|| "Vertices are less than 3!".to_string())?;
    // create comp for mesh spawning
    let mesh_handle = meshes.add(generated_mesh);
    let mut polygonal_mesh = PolygonalMesh {
        mesh_polygon: create_mode_state.mesh_polygon.clone(),
        mesh_handle: Some(mesh_handle.clone()),
    };
    let manual_mesh_material = materials.add(Color::rgba(0.8, 0.7, 0.6, 1.0).into());
    let highlight_mat_kind = HighlightKind::<StandardMaterial>::Fixed(manual_mesh_material.clone());
    let new_mesh_entity = commands
        .spawn(MaterialMeshBundle {
            mesh: mesh_handle,
            material: manual_mesh_material.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        })
        .insert(polygonal_mesh)
        .insert(PickableBundle::default())
        .insert(Highlight::<StandardMaterial> {
            hovered: Some(highlight_mat_kind.clone()),
            pressed: Some(highlight_mat_kind.clone()),
            selected: Some(highlight_mat_kind.clone()),
        })
        .id();
    // add new mesh as child of canvas
    commands.entity(canvas_entity).add_child(new_mesh_entity);
    // reset polygon state
    create_mode_state.mesh_polygon.clear_with_reset();
    Ok(())
}

/// Mark all temporarily created create mode entities for cleanup.
fn cleanup_create_mode_entities(
    commands: &mut Commands,
    query_edge_indicators: &Query<Entity, (With<EdgeIndicator>, Without<Cleanup>)>,
    query_indicators: &Query<Entity, (With<VertexIndicator>, Without<Cleanup>)>,
) {
    // mark edge/vertex indicators for de-spawn
    for entity in query_edge_indicators.iter() {
        commands.entity(entity).insert(Cleanup::Recursive);
    }
    for entity in query_indicators.iter() {
        commands.entity(entity).insert(Cleanup::Recursive);
    }
}
