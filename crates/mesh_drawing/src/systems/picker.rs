use bevy::prelude::*;
use bevy_mod_picking::{HoverEvent, PickableMesh, PickingEvent, SelectionEvent};
use bevy_mod_raycast::Intersection;

use crate::{
    components::{Canvas, EdgeIndicator, PolygonalMesh, PolygonalMeshIndicators, VertexIndicator},
    events::edit_mode::{EditModeEvent, InsertVertexData},
    resources::PluginSettings,
    utils::canvas_correction::get_canvas_corrected_translation,
};

use super::raycast::MeshDrawingRaycastSet;

/// Handle's the mod/entity picker events.
#[allow(clippy::too_many_arguments)]
pub fn handle_picker_events(
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventReader<PickingEvent>,
    mut edit_mode_event: EventWriter<EditModeEvent>,
    plugin_settings: Res<PluginSettings>,
    query_canvas: Query<&Transform, With<Canvas>>,
    query_intersections: Query<&Intersection<MeshDrawingRaycastSet>>,
    query_mesh_with_indicators: Query<&PolygonalMeshIndicators, With<PickableMesh>>,
    query_mesh_without_indicators: Query<
        &PolygonalMesh,
        (With<PickableMesh>, Without<PolygonalMeshIndicators>),
    >,
    query_vertex_indicators: Query<&VertexIndicator>,
    query_edge_indicators: Query<&EdgeIndicator>,
) {
    let Ok(canvas_transform) = query_canvas.get_single() else { return; };
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => match e {
                SelectionEvent::JustSelected(entity) => {}
                SelectionEvent::JustDeselected(entity) => {}
            },
            PickingEvent::Hover(e) => match e {
                HoverEvent::JustEntered(entity) => {}
                HoverEvent::JustLeft(entity) => {}
            },
            PickingEvent::Clicked(entity) => {
                info!("Clicked entity: {:?}", entity);
                if query_canvas.contains(*entity) {
                    // if canvas is clicked cleanup every thing
                    edit_mode_event.send(EditModeEvent::Reset);
                } else if query_mesh_with_indicators.contains(*entity) {
                    // if current mesh (with active indicators) is clicked.
                    // Do nothing for now
                } else if query_mesh_without_indicators.contains(*entity) {
                    // if a new mesh (without active indicators) is clicked.
                    edit_mode_event.send(EditModeEvent::PolygonalMeshSelect(*entity));
                } else if query_vertex_indicators.contains(*entity) {
                    // if vertex indicator is clicked
                    if plugin_settings.is_edit_mode_remove_vertex_enabled
                        && keyboard_input.pressed(KeyCode::LAlt)
                    {
                        edit_mode_event.send(EditModeEvent::VertexRemove(*entity));
                    }
                } else if query_edge_indicators.contains(*entity) {
                    let Ok(EdgeIndicator(edge))  = query_edge_indicators.get(*entity) else { continue; };
                    // if edge indicator is clicked
                    if plugin_settings.is_edit_mode_insert_vertex_enabled
                        && keyboard_input.pressed(KeyCode::LControl)
                    {
                        let Ok(intersection) = query_intersections.get_single() else { continue; };
                        let Some(intersection_pos) = intersection.position() else { continue; };
                        let intersection_pos =
                            get_canvas_corrected_translation(*intersection_pos, canvas_transform);
                        edit_mode_event.send(EditModeEvent::VertexInsert(InsertVertexData {
                            edge: edge.clone(),
                            translation: intersection_pos,
                        }));
                    }
                } else {
                    // For any other reset everything
                    edit_mode_event.send(EditModeEvent::Reset);
                }
            }
        }
    }
}
