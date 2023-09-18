use bevy::prelude::{Vec2, Vec3};
use bevy_prototype_lyon::prelude::tess::path::EndpointId;
use bevy_prototype_lyon::prelude::tess::{
    BuffersBuilder, FillTessellator, FillVertex, VertexBuffers,
};
use bevy_prototype_lyon::prelude::{FillOptions, PathBuilder};

type E3 = Vec3;

/// Generate/Build Mesh using Lyon library.

/// Generated Mesh Struct
#[derive(Debug, Default)]
pub struct GeneratedMesh<V> {
    pub vertices: Option<Vec<V>>,
    pub indices: Option<Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Face {
    Top,
    Bottom,
}

#[derive(Debug)]
struct MyPathPoint {
    id: Option<EndpointId>,
    position: Vec3,
    face: Face,
}

/// Generate/Build Mesh using Lyon library. WIP
pub fn get_lyon_basic_geometry() -> GeneratedMesh<[f32; 3]> {
    // later change these to input args ---------- START
    let base_2d_path_array = vec![
        (1.0, -1.0),
        (-1.0, -1.0),
        (-1.0, 1.0),
        (1.0, 1.0),
        (2.0, 2.0),
    ];
    let extrude_amount = 1.;
    // later change these to input args ---------- END

    let mut generated_mesh = GeneratedMesh::default();
    let mut path_builder = PathBuilder::new();

    // build path from vec of 2d points.
    for (idx, point) in base_2d_path_array.iter().enumerate() {
        let point_vec = Vec2::new(point.0, point.1);
        if idx == 0 {
            // first point
            path_builder.move_to(point_vec);
        } else if idx == base_2d_path_array.len() - 1 {
            // last point
            path_builder.line_to(point_vec);
            path_builder.close();
        } else {
            // intermediate points
            path_builder.line_to(point_vec);
        }
    }
    let path = path_builder.build();

    // Will contain the result of the tessellation.
    let mut geometry: VertexBuffers<MyPathPoint, u16> = VertexBuffers::new();
    let mut tessellator = FillTessellator::new();
    let mut extra_indices = vec![];
    {
        // bottom faces.
        let mut bottom_buffers_builder =
            BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
                let pos = vertex.position().to_array();
                MyPathPoint {
                    id: vertex.as_endpoint_id(),
                    position: Vec3::new(pos[0], 0., pos[1]),
                    face: Face::Bottom,
                }
            });
        // Compute the tessellation.
        tessellator
            .tessellate_path(
                &path.0,
                &FillOptions::default(),
                &mut bottom_buffers_builder,
            )
            .unwrap();
        // top faces.
        let mut top_buffers_builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            let pos = vertex.position().to_array();
            MyPathPoint {
                id: vertex.as_endpoint_id(),
                position: Vec3::new(pos[0], extrude_amount, pos[1]),
                face: Face::Top,
            }
        });
        tessellator
            .tessellate_path(&path.0, &FillOptions::default(), &mut top_buffers_builder)
            .unwrap();
        // for intermediate faces
        let mut bottom_points = vec![];
        let mut top_points = vec![];
        for idx in 0..geometry.vertices.len() {
            let vert = &geometry.vertices[idx];
            match vert.face {
                Face::Top => top_points.push(idx),
                Face::Bottom => bottom_points.push(idx),
            }
        }
        println!("bottom_points: {:?}", bottom_points);
        println!("top_points: {:?}", top_points);
        for idx in 0..(bottom_points.len() - 1) {
            let b_pt = bottom_points.get(idx).unwrap();
            let b_pt_next = bottom_points.get(idx + 1).unwrap();
            let t_pt = top_points.get(idx).unwrap();
            let t_pt_next = top_points.get(idx + 1).unwrap();
            // add triangle 1
            extra_indices.push(*b_pt);
            extra_indices.push(*t_pt);
            extra_indices.push(*b_pt_next);
            // add triangle 2
            // extra_indices.push(*b_pt_next);
            // extra_indices.push(*t_pt_next);
            // extra_indices.push(*t_pt);
        }
    }

    println!("geometry: {:?}", geometry);
    // Assign the verts and indices of the geometry to the gen mesh
    generated_mesh.vertices = Some(
        geometry
            .vertices
            .iter()
            .map(|point| point.position.to_array())
            .collect(),
    );
    // Add the indices in geometry.
    extra_indices.extend_from_slice(
        &geometry
            .indices
            .iter()
            .map(|i| *i as usize)
            .collect::<Vec<_>>(),
    );
    generated_mesh.indices = Some(extra_indices);

    generated_mesh
}
