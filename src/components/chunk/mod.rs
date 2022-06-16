use self::{
    chunk_state::{ChunkState, ChunkStateComponent},
    pos::PosComponent,
    voxel::{voxels_to_vertex::append_vertex, Voxel},
};
use crate::{components::static_mesh::vertex::Vertex, resources::generator::GeneratorResource};
use bevy::prelude::*;

use super::static_mesh::StaticMeshComponent;

pub mod chunk_state;
pub mod pos;
pub mod voxel;

pub const CHUNK_REAL_SIZE: usize = 16;
pub const CHUNK_VOXELS_SIZE: usize = CHUNK_REAL_SIZE + 1;
pub const CHUNK_VOXELS_VOLUME: usize = CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE;

#[derive(Component)]
pub struct ChunkComponent {
    voxels: [Voxel; CHUNK_VOXELS_VOLUME],
}

impl ChunkComponent {
    pub fn new() -> Self {
        Self {
            voxels: [Voxel {
                value: 0.,
                color: Color::rgb(0., 0., 0.),
            }; CHUNK_VOXELS_VOLUME],
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        generator: &GeneratorResource,
        pos: PosComponent,
    ) {
        let mut chunk = ChunkComponent::new();
        chunk.generate(pos.clone(), generator);
        let vertices = chunk.generate_vertices(pos.clone());
        println!("vertices size: {}", vertices.len());

        let mesh = StaticMeshComponent::spawn(commands, meshes, materials, vertices);

        commands
            .spawn()
            .insert(chunk)
            .insert(pos)
            .insert(ChunkStateComponent(ChunkState::NotInitialized))
            .add_child(mesh);
    }

    pub fn generate_vertices(&mut self, pos: PosComponent) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..CHUNK_REAL_SIZE {
            for y in 0..CHUNK_REAL_SIZE {
                for z in 0..CHUNK_REAL_SIZE {
                    append_vertex(
                        PosComponent::new(x as i64, y as i64, z as i64),
                        self,
                        &mut vertices,
                    );
                }
            }
        }

        for v in vertices.iter_mut() {
            v.pos = Vec3::new(
                v.pos.x + (pos.x * CHUNK_REAL_SIZE as i64) as f32,
                v.pos.y + (pos.y * CHUNK_REAL_SIZE as i64) as f32,
                v.pos.z + (pos.z * CHUNK_REAL_SIZE as i64) as f32,
            )
        }

        vertices
    }

    pub fn generate_voxels(&mut self, pos: PosComponent, generator: &GeneratorResource) {
        let offset = Vec3::new(
            (pos.x * CHUNK_REAL_SIZE as i64) as f32,
            (pos.y * CHUNK_REAL_SIZE as i64) as f32,
            (pos.z * CHUNK_REAL_SIZE as i64) as f32,
        );

        generator.generate_voxels(offset, &mut self.voxels, CHUNK_VOXELS_SIZE)
    }

    pub fn generate(&mut self, pos: PosComponent, generator: &GeneratorResource) {
        self.generate_voxels(pos, generator);
    }

    pub fn check_pos_in_chunk(pos: PosComponent) -> bool {
        return pos.x >= 0
            && pos.x < CHUNK_VOXELS_SIZE as i64
            && pos.y >= 0
            && pos.y < CHUNK_VOXELS_SIZE as i64
            && pos.z >= 0
            && pos.z < CHUNK_VOXELS_SIZE as i64;
    }

    pub fn pos_to_index(pos: PosComponent) -> Option<usize> {
        if !Self::check_pos_in_chunk(pos) {
            return None;
        }
        return Some(
            pos.x as usize
                + pos.y as usize * CHUNK_VOXELS_SIZE
                + pos.z as usize * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE,
        );
    }

    fn cord_to_chunk_cord(v: i64) -> i64 {
        if v < 0 {
            return v / CHUNK_REAL_SIZE as i64 - 1;
        } else {
            return v / CHUNK_REAL_SIZE as i64;
        }
    }

    pub fn get_voxel(&self, in_chunk_position: PosComponent) -> Option<Voxel> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                return Some(self.voxels[index]);
            }
            _ => None,
        }
    }
}
