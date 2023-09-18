use bevy::prelude::{Mesh, Vec2, Vec3};
use geo::{coord, LineString, Polygon};
use mesh::MeshBuilder;

pub mod earcutr;
pub mod mesh;

/// Generate/Build Mesh using Earcutr library.

/// Generate 3D Mesh using Earcutr.
///
/// Generates a Bevy mesh given the 2D path (of points) and
/// extrude amount.
///
pub fn generate_mesh_earcutr(path_2d: Vec<Vec2>, extrude_amount: f32) -> Mesh {
    let down = Vec3::NEG_Y;
    let up = Vec3::new(0.0, 1.0, 0.0);

    let y1 = 0.;
    let y2 = extrude_amount;

    let mut builder = MeshBuilder::new();
    let polygon = Polygon::new(
        LineString::new(
            path_2d
                .iter()
                .map(|p| coord! {x: p.x as f64, y: p.y as f64})
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    // Floor
    builder.triangulate_polygon(&polygon, y1, down);

    // Ceiling
    builder.triangulate_polygon(&polygon, y2, up);

    // For every line along the polygon, add a rectangular wall
    for line in polygon.exterior().lines() {
        let corner1 = Vec3::new(line.start.x as f32, y1, line.start.y as f32);
        let corner2 = Vec3::new(line.end.x as f32, y1, line.end.y as f32);
        let corner3 = Vec3::new(line.end.x as f32, y2, line.end.y as f32);
        let corner4 = Vec3::new(line.start.x as f32, y2, line.start.y as f32);

        // Now let's go fetch our buddy Norm
        let bottom_line = corner2 - corner1;
        let up_line = corner3 - corner2;
        let normal = bottom_line.cross(up_line).normalize();

        builder.add_quad([corner1, corner2, corner3, corner4], normal);
    }
    // TODO Interiors. The normal is reversed

    builder.build()
}
