use bevy::prelude::*;
use bevy_mod_raycast::prelude::RaycastSource;

use crate::{
    components::{Canvas, GrabTransformable},
    resources::{drawing::EditModeState, DrawingMode, DrawingState},
    utils::canvas_correction::get_canvas_corrected_translation,
};

use super::raycast::{get_first_intersection_data_for_source, MeshDrawingRaycastSet};

pub fn handle_vertex_indicator_grab(
    drawing_state: Res<DrawingState>,
    mut query_indicators: Query<&mut Transform, (With<GrabTransformable>, Without<Canvas>)>,
    query_intersections: Query<&RaycastSource<MeshDrawingRaycastSet>>,
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

    let Some((_, intersection)) = get_first_intersection_data_for_source(&query_intersections)
    else {
        return;
    };
    let position = get_canvas_corrected_translation(intersection.position(), canvas_transform);
    transform.translation = position;
}
