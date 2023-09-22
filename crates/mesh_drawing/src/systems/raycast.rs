use bevy::{prelude::*, reflect::TypePath};
use bevy_mod_picking::prelude::{Highlight, HighlightKind, PickableBundle, RaycastPickCamera};
use bevy_mod_raycast::{
    prelude::{Raycast, RaycastMesh, RaycastMethod, RaycastPluginState, RaycastSource},
    IntersectionData,
};

use crate::{
    components::{Canvas, MeshDrawingCamera, VertexIndicator},
    events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent},
    utils::canvas_correction::get_canvas_corrected_translation,
};

/// Unit Struct use to mark the main mesh drawing
/// ray-cast set as a part of the same group.
#[derive(TypePath)]
pub struct MeshDrawingRaycastSet;

/// Unit Struct use to mark the vertex grabbing
/// ray-cast set as a part of the same group.
#[derive(TypePath)]
pub struct VertexGrabbingRaycastSet;

pub fn setup_raycast(mut commands: Commands) {
    // Overwrite the default plugin state with one that enables the debug cursor. This line can be
    // removed if the debug cursor isn't needed as the state is set to default values when the
    // default plugin is added.
    commands.insert_resource(
        RaycastPluginState::<MeshDrawingRaycastSet>::default().with_debug_cursor(),
    );
    commands.insert_resource(
        RaycastPluginState::<VertexGrabbingRaycastSet>::default().with_debug_cursor(),
    );
}

// Update our `RaycastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MeshDrawingRaycastSet>>,
    mut query_grab_sources: Query<&mut RaycastSource<VertexGrabbingRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
    for mut pick_source in &mut query_grab_sources {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

pub fn enable_raycast_on_canvas_add(
    mut commands: Commands,
    query: Query<(Entity, &Handle<StandardMaterial>), Added<Canvas>>,
) {
    for (entity, material) in query.iter() {
        let highlight_mat_kind = HighlightKind::<StandardMaterial>::Fixed(material.clone());
        commands.entity(entity).insert((
            PickableBundle::default(),
            RaycastMesh::<MeshDrawingRaycastSet>::default(),
            Highlight::<StandardMaterial> {
                hovered: Some(highlight_mat_kind.clone()),
                pressed: Some(highlight_mat_kind.clone()),
                selected: Some(highlight_mat_kind.clone()),
            },
        ));
    }
}

pub fn disable_raycast_on_canvas_remove(
    mut commands: Commands,
    mut removed: RemovedComponents<Canvas>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove::<PickableBundle>()
            .remove::<Highlight<StandardMaterial>>()
            .remove::<RaycastMesh<MeshDrawingRaycastSet>>()
            .remove::<Highlight<StandardMaterial>>();
    }
}

pub fn enable_raycast_on_vertex_indicators_add(
    mut commands: Commands,
    query: Query<Entity, Added<VertexIndicator>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RaycastMesh::<VertexGrabbingRaycastSet>::default());
    }
}

pub fn enable_raycast_on_camera_add(
    mut commands: Commands,
    query: Query<Entity, Added<MeshDrawingCamera>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RaycastSource::<MeshDrawingRaycastSet>::new())
            .insert(RaycastSource::<VertexGrabbingRaycastSet>::new())
            .insert(RaycastPickCamera::default());
    }
}

pub fn disable_raycast_on_camera_remove(
    mut commands: Commands,
    mut removed: RemovedComponents<MeshDrawingCamera>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove::<RaycastSource<MeshDrawingRaycastSet>>()
            .remove::<RaycastSource<VertexGrabbingRaycastSet>>()
            .remove::<RaycastPickCamera>();
    }
}

/// Handle raycast intersections.
///
/// Dispatch `CreateModeEvent` on user interactions along with intersections data.
#[allow(clippy::too_many_arguments)]
pub fn handle_raycast_intersections(
    mut create_mode_event: EventWriter<CreateModeEvent>,
    mouse_btn_input: Res<Input<MouseButton>>,
    mut query_intersections: Query<&RaycastSource<MeshDrawingRaycastSet>>,
) {
    if mouse_btn_input.just_pressed(MouseButton::Left) {
        // Add new vertex

        let Some((entity, intersection)) =
            get_first_intersection_data_for_source(&query_intersections)
        else {
            return;
        };

        let intersection_point = intersection.position();
        info!("intersection_point: {:?}", intersection_point);
        create_mode_event.send(CreateModeEvent::VertexAdd(intersection_point));
    } else if mouse_btn_input.just_pressed(MouseButton::Right) {
        create_mode_event.send(CreateModeEvent::PolygonCloseAndIntoMeshExtrude);
    }
}

pub fn get_first_intersection_data_for_source<T: TypePath>(
    query_intersections: &Query<&RaycastSource<T>>,
) -> Option<(Entity, IntersectionData)> {
    for source in query_intersections.iter() {
        for (i, (entity, intersection_data)) in source.intersections().iter().enumerate() {
            if i == 0 {
                return Some((*entity, intersection_data.clone()));
            }
        }
    }
    None
}

pub fn get_multi_intersection_data_for_source<T: TypePath>(
    query_intersections: &Query<&RaycastSource<T>>,
) -> Vec<(Entity, IntersectionData)> {
    let mut intersections = vec![];
    for source in query_intersections.iter() {
        for (i, (entity, intersection_data)) in source.intersections().iter().enumerate() {
            if i == 0 {
                intersections.push((*entity, intersection_data.clone()));
            }
        }
    }
    intersections
}

/// Handle raycast intersections for vertex grabbing.
///
/// Dispatch `EditModeEvent` on user interactions along with intersections data.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_vertex_grabbing_raycast_intersections(
    mut edit_mode_event: EventWriter<EditModeEvent>,
    mouse_btn_input: Res<Input<MouseButton>>,
    query_intersections: Query<&RaycastSource<VertexGrabbingRaycastSet>>,
    query_meshes: Query<
        (Entity, &Transform),
        (With<RaycastMesh<VertexGrabbingRaycastSet>>, Without<Canvas>),
    >,
    query_canvas: Query<&Transform, With<Canvas>>,
) {
    let Ok(canvas_transform) = query_canvas.get_single() else {
        return;
    };
    let mut intersections_pos_dist = vec![];
    let multi_intersection_data = get_multi_intersection_data_for_source(&query_intersections);
    for (_, intersections) in multi_intersection_data.iter() {
        intersections_pos_dist.push((intersections.distance(), intersections.position()));
    }
    intersections_pos_dist.sort_by(|i1, i2| i1.0.total_cmp(&i2.0));
    let mut closest_entity = None;
    if let Some((_, position)) = intersections_pos_dist.first().cloned() {
        let position = get_canvas_corrected_translation(position, canvas_transform);
        let mut min_dist = f32::MAX;
        for (entity, Transform { translation, .. }) in query_meshes.iter() {
            let dist = translation.distance_squared(position);
            if dist < min_dist {
                closest_entity = Some(entity);
                min_dist = dist;
            }
        }
    }
    let Some(closest_entity) = closest_entity else {
        return;
    };
    if mouse_btn_input.just_pressed(MouseButton::Left) {
        edit_mode_event.send(EditModeEvent::VertexIndicatorJustPressed(closest_entity));
    } else if mouse_btn_input.just_released(MouseButton::Left) {
        edit_mode_event.send(EditModeEvent::VertexIndicatorJustReleased);
    }
}
