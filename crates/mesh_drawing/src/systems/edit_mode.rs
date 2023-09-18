use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_mod_picking::PickableMesh;
use mesh_geometry_utils::data_structures::Edge;

use crate::{
    components::{
        Canvas, Cleanup, EdgeIndicator, GrabTransformable, PolygonalMesh, PolygonalMeshIndicators,
        VertexIndicator,
    },
    events::edit_mode::{EditModeEvent, InsertVertexData},
    resources::{
        drawing::{CreateModeState, EditModeState},
        DrawingMode, DrawingState,
    },
    utils::indicators::{spawn_edge_indicator, spawn_vertex_indicators, EDGE_INDICATOR_WIDTH},
};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_edit_mode_events(
    mut events: EventReader<EditModeEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut drawing_state: ResMut<DrawingState>,
    query_canvas: Query<(Entity, &Transform), With<Canvas>>,
    query_vertex_indicators: Query<(Entity, &VertexIndicator)>,
    query_edge_indicators: Query<(Entity, &EdgeIndicator)>,
    mut query_mesh_indicators_set: ParamSet<(
        Query<(Entity, &PolygonalMeshIndicators), With<PickableMesh>>,
        Query<(&mut PolygonalMesh, &mut PolygonalMeshIndicators), With<PickableMesh>>,
    )>,
    query_mesh_without_indicators: Query<
        &PolygonalMesh,
        (With<PickableMesh>, Without<PolygonalMeshIndicators>),
    >,
) {
    let Ok((canvas_entity, canvas_transform)) = query_canvas.get_single() else { return; };
    for event in events.iter() {
        let DrawingMode::EditMode(edit_mode_state) = &mut drawing_state.mode else { return; };
        match event {
            EditModeEvent::PolygonalMeshSelect(entity) => {
                // cleanup existing...
                let query_mesh_w_indicators_for_cleanup = query_mesh_indicators_set.p0();
                cleanup_edit_mode_entities_and_reset(
                    &mut commands,
                    edit_mode_state,
                    &query_mesh_w_indicators_for_cleanup,
                );
                // Create new...
                if let Ok(PolygonalMesh { mesh_polygon, .. }) =
                    query_mesh_without_indicators.get(*entity)
                {
                    let mut indicators = PolygonalMeshIndicators {
                        edges: vec![],
                        vertices: vec![],
                    };
                    for (id, vertex) in mesh_polygon.vertices.enumerate() {
                        let entity = spawn_vertex_indicators(
                            *vertex,
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                        );
                        commands.entity(entity).insert(VertexIndicator(id));
                        indicators.vertices.push(entity)
                    }
                    // spawn edge indicators
                    for Edge { from, to } in mesh_polygon.edges.iter().cloned() {
                        if let (Some(from_vert), Some(to_vert)) = (
                            mesh_polygon.vertices.get(from),
                            mesh_polygon.vertices.get(to),
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
                            indicators.edges.push(entity);
                        }
                    }
                    // Push the vertex/edge indicators as children of selected mesh
                    // and insert the component for future ref
                    commands
                        .entity(*entity)
                        .push_children(&indicators.vertices)
                        .push_children(&indicators.edges)
                        .insert(indicators);
                    // Set active mesh
                    edit_mode_state.active_mesh = Some(*entity);
                }
            }
            EditModeEvent::VertexIndicatorJustPressed(entity) => {
                // Set active entity
                edit_mode_state.active_vertex_indicator = Some(*entity);
                // mark as gizmo transformable
                commands.entity(*entity).insert(GrabTransformable);
                // Deactivate rest of indicators
                for (indicator_entity, _) in
                    query_vertex_indicators.iter().filter(|(e, _)| e != entity)
                {
                    // make non transformable
                    commands
                        .entity(indicator_entity)
                        .remove::<GrabTransformable>();
                }
            }
            EditModeEvent::VertexIndicatorJustReleased => {
                // Deactivate all of indicators
                for (indicator_entity, _) in query_vertex_indicators.iter() {
                    // make non transformable
                    commands
                        .entity(indicator_entity)
                        .remove::<GrabTransformable>();
                }
                // Unset active entity
                edit_mode_state.active_vertex_indicator = None;
            }
            EditModeEvent::Reset => {
                let query_mesh_w_indicators_for_cleanup = query_mesh_indicators_set.p0();
                cleanup_edit_mode_entities_and_reset(
                    &mut commands,
                    edit_mode_state,
                    &query_mesh_w_indicators_for_cleanup,
                );
            }
            EditModeEvent::CreateModeSwitch => {
                let query_mesh_w_indicators_for_cleanup = query_mesh_indicators_set.p0();
                cleanup_edit_mode_entities_and_reset(
                    &mut commands,
                    edit_mode_state,
                    &query_mesh_w_indicators_for_cleanup,
                );
                // switch to create mode.
                drawing_state.mode = DrawingMode::CreateMode(CreateModeState::default());
            }
            EditModeEvent::VertexInsert(InsertVertexData { edge, translation }) => {
                let Some(active_mesh) = edit_mode_state.active_mesh else { continue; };
                let mut query_mesh_with_indicators = query_mesh_indicators_set.p1();
                let Ok((mut polygonal_mesh, mut polygonal_mesh_indicators)) = query_mesh_with_indicators.get_mut(active_mesh) else { continue; };
                // insert vertex in MeshPolygon ds
                let Some(vertex_id) = polygonal_mesh
                    .mesh_polygon
                    .insert_vertex_on_edge(translation.xz(), edge.clone()) else { continue; };
                // cleanup existing edge indicator
                for (entity, indicator) in query_edge_indicators.iter() {
                    if indicator.0 == edge.clone() {
                        // remove edge from indicators list
                        polygonal_mesh_indicators.edges.retain(|e| *e != entity);
                        // mark for cleanup
                        commands.entity(entity).insert(Cleanup::Recursive);
                    }
                }
                // Draw new vertex indicator
                let entity = spawn_vertex_indicators(
                    translation.xz(),
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
                // Mark as indicator
                commands.entity(entity).insert(VertexIndicator(vertex_id));
                // push vertex indicator as child of active mesh
                commands.entity(active_mesh).add_child(entity);
                // add to the indicators list
                polygonal_mesh_indicators.vertices.push(entity);
                // find the new edge inserted (if any) & draw it
                for Edge { from, to } in polygonal_mesh
                    .mesh_polygon
                    .edges
                    .filter_by_vertex(vertex_id)
                    .cloned()
                {
                    if let (Some(from_vert), Some(to_vert)) = (
                        polygonal_mesh.mesh_polygon.vertices.get(from),
                        polygonal_mesh.mesh_polygon.vertices.get(to),
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
                        // push edge indicator as child of active mesh
                        commands.entity(active_mesh).add_child(entity);
                        // add to list
                        polygonal_mesh_indicators.edges.push(entity);
                    }
                }
                // regenerate mesh and assign it to existing...
                let Some(new_mesh) = polygonal_mesh.mesh_polygon.extrude_to_bevy_mesh(2.0) else { error!("Could not extrude mesh!"); return; };
                if let Some(mesh_handle) = polygonal_mesh.mesh_handle.clone() {
                    if let Some(mesh) = meshes.get_mut(&mesh_handle) {
                        info!("Generating new mesh...");
                        mesh.clone_from(&new_mesh);
                    }
                }
            }
            EditModeEvent::VertexRemove(entity) => {
                // get vertex id
                let Ok((_, VertexIndicator(vertex_id))) = query_vertex_indicators.get(*entity) else { continue; };
                // get mut polygonal mesh & indicators components for active mesh...
                let Some(active_mesh) = edit_mode_state.active_mesh else { continue; };
                let mut query_mesh_with_indicators = query_mesh_indicators_set.p1();
                let Ok((mut polygonal_mesh, mut polygonal_mesh_indicators)) = query_mesh_with_indicators.get_mut(active_mesh) else { continue; };
                if polygonal_mesh.mesh_polygon.vertices.len() <= 3 {
                    error!("Cannot remove vertices! vertices are less than or equal to 3!");
                    continue;
                }
                // remove the vertex from MeshPolygon ds
                let (Some(_), added_edges) = polygonal_mesh.mesh_polygon.remove_vertex(*vertex_id) else { continue; };
                // cleanup connected edge indicators...
                for (entity, EdgeIndicator(Edge { from, to })) in query_edge_indicators.iter() {
                    if from == vertex_id || to == vertex_id {
                        // remove edge from indicators list
                        polygonal_mesh_indicators.edges.retain(|e| *e != entity);
                        commands.entity(entity).insert(Cleanup::Recursive);
                    }
                }
                // cleanup vertex indicator...
                // remove from indicators list
                polygonal_mesh_indicators.vertices.retain(|e| e != entity);
                commands.entity(*entity).insert(Cleanup::Recursive);
                // Draw the added edges...
                for Edge { from, to } in added_edges {
                    if let (Some(from_vert), Some(to_vert)) = (
                        polygonal_mesh.mesh_polygon.vertices.get(from),
                        polygonal_mesh.mesh_polygon.vertices.get(to),
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
                        // push edge indicator as child of active mesh
                        commands.entity(active_mesh).add_child(entity);
                        // add to list
                        polygonal_mesh_indicators.edges.push(entity);
                    }
                }
                // regenerate mesh and assign it to existing...
                let Some(new_mesh) = polygonal_mesh.mesh_polygon.extrude_to_bevy_mesh(2.0) else { error!("Could not extrude mesh!"); return; };
                if let Some(mesh_handle) = polygonal_mesh.mesh_handle.clone() {
                    if let Some(mesh) = meshes.get_mut(&mesh_handle) {
                        info!("Generating new mesh...");
                        mesh.clone_from(&new_mesh);
                    }
                }
            }
        }
    }
}

/// Mark all temporarily created edit mode entities for cleanup and reset state.
fn cleanup_edit_mode_entities_and_reset(
    commands: &mut Commands,
    edit_mode_state: &mut EditModeState,
    query_mesh_w_indicators_for_cleanup: &Query<
        (Entity, &PolygonalMeshIndicators),
        With<PickableMesh>,
    >,
) {
    for (entity, PolygonalMeshIndicators { vertices, edges }) in
        query_mesh_w_indicators_for_cleanup.iter()
    {
        // cleanup the indicators comp on the mesh entity.
        commands.entity(entity).remove::<PolygonalMeshIndicators>();
        // cleanup the indicator(s).
        for entity in vertices {
            commands.entity(*entity).insert(Cleanup::Recursive);
        }
        for entity in edges {
            commands.entity(*entity).insert(Cleanup::Recursive);
        }
    }
    // Deactivate mesh and indicator
    edit_mode_state.active_mesh = None;
    edit_mode_state.active_vertex_indicator = None;
}

/// Handle active vertex indicator.
pub fn handle_active_indicator(
    query_moved_indicators: Query<(&Transform, &VertexIndicator), Changed<Transform>>,
    mut query_with_indicators: Query<
        &mut PolygonalMesh,
        (With<PickableMesh>, Without<VertexIndicator>),
    >,
    mut query_edge_indicators: Query<(&mut Transform, &EdgeIndicator), Without<VertexIndicator>>,
    drawing_state: Res<DrawingState>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (active_mesh, active_vertex_indicator) = if let DrawingMode::EditMode(EditModeState {
        active_mesh: Some(active_mesh),
        active_vertex_indicator: Some(active_vertex_indicator),
    }) = drawing_state.mode
    {
        (active_mesh, active_vertex_indicator)
    } else {
        return;
    };
    let Ok((transform, VertexIndicator(vertex_id)))= query_moved_indicators.get(active_vertex_indicator) else {
            return;
        };
    let Ok(mut polygonal_mesh) = query_with_indicators.get_mut(active_mesh) else {
            return;
        };
    if let Some(vertex) = polygonal_mesh.mesh_polygon.vertices.get_mut(*vertex_id) {
        // Manipulate vertex in path 2d...
        vertex.x = transform.translation.x;
        // since y is vertical we use z...
        vertex.y = transform.translation.z;
        // regenerate mesh and assign it to existing...
        let Some(new_mesh) = polygonal_mesh.mesh_polygon.extrude_to_bevy_mesh(2.0) else { error!("Could not extrude mesh!"); return; };
        if let Some(mesh_handle) = polygonal_mesh.mesh_handle.clone() {
            if let Some(mesh) = meshes.get_mut(&mesh_handle) {
                info!("Generating new mesh...");
                mesh.clone_from(&new_mesh);
            }
        }
    }
    // move edge indicators accordingly
    for (mut transform, EdgeIndicator(Edge { from, to })) in query_edge_indicators.iter_mut() {
        if from == vertex_id || to == vertex_id {
            if let (Some(from_vert), Some(to_vert)) = (
                polygonal_mesh.mesh_polygon.vertices.get(*from).cloned(),
                polygonal_mesh.mesh_polygon.vertices.get(*to).cloned(),
            ) {
                // calc for new transform and set it...
                let mid_pt = (to_vert + from_vert) / 2.;
                let edge_vec = to_vert - from_vert;
                let edge_rot_angle_y = edge_vec.angle_between(Vec2::X);
                *transform = Transform {
                    translation: Vec3::new(mid_pt.x, 0., mid_pt.y),
                    rotation: Quat::from_rotation_y(edge_rot_angle_y),
                    scale: Vec3::new(
                        edge_vec.length() / 2.,
                        EDGE_INDICATOR_WIDTH,
                        EDGE_INDICATOR_WIDTH,
                    ),
                };
            }
        }
    }
}
