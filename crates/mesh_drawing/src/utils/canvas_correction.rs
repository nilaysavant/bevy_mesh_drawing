use bevy::prelude::*;

/// Get corrected value of translation.
///
/// Used to convert translation from global to the local wrt canvas as most
/// entities are children of the canvas.
pub fn get_canvas_corrected_translation(translation: Vec3, canvas_transform: &Transform) -> Vec3 {
    let inverted_canvas_rot = canvas_transform.rotation.inverse();
    let adjusted_translation = translation - canvas_transform.translation;
    inverted_canvas_rot * adjusted_translation
}
