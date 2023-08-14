use bevy::prelude::*;

use crate::common::components::static_mesh::{vertex::Vertex, StaticMeshComponent};

pub struct ObjectHandler {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}
impl ObjectHandler {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

#[derive(Resource)]
pub struct ObjectHandlers {
    pub handlers: Vec<ObjectHandler>,
}

impl ObjectHandlers {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Self {
        Self {
            handlers: vec![ObjectHandler::new(
                meshes.add(StaticMeshComponent::generate_mesh(vec![
                    Vertex {
                        color: Color::WHITE,
                        normal: Vec3::Y,
                        pos: Vec3::new(0., 0., 0.),
                    },
                    Vertex {
                        color: Color::WHITE,
                        normal: Vec3::Y,
                        pos: Vec3::new(1., 0., 0.),
                    },
                    Vertex {
                        color: Color::WHITE,
                        normal: Vec3::Y,
                        pos: Vec3::new(0., 1., 0.),
                    },
                ])),
                materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: Color::rgb(0., 0., 1.),
                    metallic: 1.,
                    reflectance: 1.,
                    double_sided: true,
                    ..default()
                }),
            )],
        }
    }
}
