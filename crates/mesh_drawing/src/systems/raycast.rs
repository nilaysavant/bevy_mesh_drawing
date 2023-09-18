use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_mod_picking::{Highlighting, PickableBundle, PickingCameraBundle};
use bevy_mod_raycast::{
    DefaultPluginState, Intersection, RayCastMesh, RayCastMethod, RayCastSource,
};

use crate::{
    components::{Canvas, MeshDrawingCamera, VertexIndicator},
    events::{create_mode::CreateModeEvent, edit_mode::EditModeEvent},
    utils::canvas_correction::get_canvas_corrected_translation,
};

/// Unit Struct use to mark the main mesh drawing
/// ray-cast set as a part of the same group.
pub struct MeshDrawingRaycastSet;

/// Unit Struct use to mark the vertex grabbing
/// ray-cast set as a part of the same group.
pub struct VertexGrabbingRaycastSet;

pub fn setup_raycast(mut commands: Commands) {
    // Overwrite the default plugin state with one that enables the debug cursor. This line can be
    // removed if the debug cursor isn't needed as the state is set to default values when the
    // default plugin is added.
    commands.insert_resource(
        DefaultPluginState::<MeshDrawingRaycastSet>::default().with_debug_cursor(),
    );
    commands.insert_resource(
        DefaultPluginState::<VertexGrabbingRaycastSet>::default().with_debug_cursor(),
    );
}

// Update our `RayCastSource` with the current cursor position every frame.
pub fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MeshDrawingRaycastSet>>,
    mut query_grab_sources: Query<&mut RayCastSource<VertexGrabbingRaycastSet>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
    for mut pick_source in &mut query_grab_sources {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}

pub fn enable_raycast_on_canvas_add(
    mut commands: Commands,
    query: Query<(Entity, &Handle<StandardMaterial>), Added<Canvas>>,
) {
    for (entity, material) in query.iter() {
        commands
            .entity(entity)
            .insert_bundle(PickableBundle::default())
            .insert(Highlighting {
                initial: material.clone(),
                hovered: Some(material.clone()),
                pressed: Some(material.clone()),
                selected: Some(material.clone()),
            })
            .insert(RayCastMesh::<MeshDrawingRaycastSet>::default());
    }
}

pub fn disable_raycast_on_canvas_remove(
    mut commands: Commands,
    removed: RemovedComponents<Canvas>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove_bundle::<PickableBundle>()
            .remove::<Highlighting<StandardMaterial>>()
            .remove::<RayCastMesh<MeshDrawingRaycastSet>>();
    }
}

pub fn enable_raycast_on_vertex_indicators_add(
    mut commands: Commands,
    query: Query<Entity, Added<VertexIndicator>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RayCastMesh::<VertexGrabbingRaycastSet>::default());
    }
}

pub fn enable_raycast_on_camera_add(
    mut commands: Commands,
    query: Query<Entity, Added<MeshDrawingCamera>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert_bundle(PickingCameraBundle::default())
            .insert(RayCastSource::<MeshDrawingRaycastSet>::new())
            .insert(RayCastSource::<VertexGrabbingRaycastSet>::new());
    }
}

pub fn disable_raycast_on_camera_remove(
    mut commands: Commands,
    removed: RemovedComponents<MeshDrawingCamera>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove_bundle::<PickingCameraBundle>()
            .remove::<RayCastSource<MeshDrawingRaycastSet>>()
            .remove::<RayCastSource<VertexGrabbingRaycastSet>>();
    }
}

/// Handle raycast intersections.
///
/// Dispatch `CreateModeEvent` on user interactions along with intersections data.
#[allow(clippy::too_many_arguments)]
pub fn handle_raycast_intersections(
    mut create_mode_event: EventWriter<CreateModeEvent>,
    mouse_btn_input: Res<Input<MouseButton>>,
    query: Query<&Intersection<MeshDrawingRaycastSet>>,
) {
    if mouse_btn_input.just_pressed(MouseButton::Left) {
        // Add new vertex
        let Ok(intersection) = query.get_single() else {
            return;
        };
        let Some(intersection_point) = intersection.position() else {
            return;
        };
        info!("intersection_point: {:?}", intersection_point);
        create_mode_event.send(CreateModeEvent::VertexAdd(*intersection_point));
    } else if mouse_btn_input.just_pressed(MouseButton::Right) {
        create_mode_event.send(CreateModeEvent::PolygonCloseAndIntoMeshExtrude);
    }
}

/// Handle raycast intersections for vertex grabbing.
///
/// Dispatch `EditModeEvent` on user interactions along with intersections data.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_vertex_grabbing_raycast_intersections(
    mut edit_mode_event: EventWriter<EditModeEvent>,
    mouse_btn_input: Res<Input<MouseButton>>,
    query_intersections: Query<&Intersection<VertexGrabbingRaycastSet>>,
    query_meshes: Query<
        (Entity, &Transform),
        (With<RayCastMesh<VertexGrabbingRaycastSet>>, Without<Canvas>),
    >,
    query_canvas: Query<&Transform, With<Canvas>>,
) {
    let Ok(canvas_transform) = query_canvas.get_single() else {
        return;
    };
    let mut intersections_pos_dist = vec![];
    for intersections in query_intersections.iter() {
        if let (Some(distance), Some(position)) =
            (intersections.distance(), intersections.position())
        {
            intersections_pos_dist.push((distance, position));
        }
    }
    intersections_pos_dist.sort_by(|i1, i2| i1.0.total_cmp(&i2.0));
    let mut closest_entity = None;
    if let Some((_, position)) = intersections_pos_dist.first().cloned() {
        let position = get_canvas_corrected_translation(*position, canvas_transform);
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
