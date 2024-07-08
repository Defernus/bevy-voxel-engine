use self::vertex::Vertex;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{pbr::PbrBundle, prelude::*};

pub mod vertex;

#[derive(Component, Clone, Copy)]
pub struct StaticMeshComponent;

impl StaticMeshComponent {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        vertices: Vec<Vertex>,
    ) -> Entity {
        return commands
            .spawn(PbrBundle {
                mesh: meshes.add(Self::generate_mesh(vertices)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 1.0, 1.0),
                    perceptual_roughness: 1.,
                    metallic: 0.,
                    reflectance: 0.,
                    ..default()
                }),
                ..default()
            })
            .insert(StaticMeshComponent)
            .id();
    }

    pub fn generate_mesh(vertices: Vec<Vertex>) -> Mesh {
        let mut indices_vec = Vec::with_capacity(vertices.len());

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(vertices.len());
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices.len());
        for vertex in vertices.iter() {
            indices_vec.push(positions.len() as u32);

            positions.push(vertex.pos.into());
            normals.push(vertex.normal.into());
            colors.push(vertex.color.to_srgba().to_vec4().into());
            uvs.push([1., 1.]);
        }

        let indices = mesh::Indices::U32(indices_vec);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_indices(indices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

        mesh
    }
}
