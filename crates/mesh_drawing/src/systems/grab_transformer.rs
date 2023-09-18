use bevy::prelude::*;
use bevy_mod_raycast::Intersection;

use crate::{
    components::{Canvas, GrabTransformable},
    resources::{drawing::EditModeState, DrawingMode, DrawingState},
    utils::canvas_correction::get_canvas_corrected_translation,
};

use super::raycast::MeshDrawingRaycastSet;

pub fn handle_vertex_indicator_grab(
    drawing_state: Res<DrawingState>,
    mut query_indicators: Query<&mut Transform, (With<GrabTransformable>, Without<Canvas>)>,
    query_intersections: Query<&Intersection<MeshDrawingRaycastSet>>,
    query_canvas: Query<&Transform, With<Canvas>>,
) {
    let DrawingMode::EditMode(EditModeState {
        active_vertex_indicator: Some(active_vertex_indicator),
        ..
    }) = drawing_state.mode
    else {
        return;
    };
    let Ok(canvas_transform) = query_canvas.get_single() else {
        return;
    };
    let Ok(mut transform) = query_indicators.get_mut(active_vertex_indicator) else {
        return;
    };
    for intersection in query_intersections.iter() {
        if let Some(position) = intersection.position() {
            let position = get_canvas_corrected_translation(*position, canvas_transform);
            transform.translation = position;
        }
    }
}
