use crate::{
    common::components::{pos::PosComponent, static_mesh::vertex::Vertex},
    plugins::generator::resources::GeneratorRes,
};
use bevy::prelude::*;

use self::voxel::{voxels_to_vertex::append_vertex, Voxel};

pub mod voxel;

pub const CHUNK_REAL_SIZE: usize = 32;
pub const CHUNK_VOXELS_SIZE: usize = CHUNK_REAL_SIZE + 1;
pub const CHUNK_VOXELS_VOLUME: usize = CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE * CHUNK_VOXELS_SIZE;

#[derive(Component)]
pub struct Chunk {
    voxels: [Voxel; CHUNK_VOXELS_VOLUME],
}

impl Chunk {
    pub fn new(generator: &GeneratorRes, pos: PosComponent) -> Self {
        let mut chunk = Self {
            voxels: [Voxel {
                value: 0.,
                color: Color::rgb(0., 0., 0.),
            }; CHUNK_VOXELS_VOLUME],
        };

        chunk.generate(pos, generator);

        chunk
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

    pub fn generate_voxels(&mut self, pos: PosComponent, generator: &GeneratorRes) {
        let offset = Vec3::new(
            (pos.x * CHUNK_REAL_SIZE as i64) as f32,
            (pos.y * CHUNK_REAL_SIZE as i64) as f32,
            (pos.z * CHUNK_REAL_SIZE as i64) as f32,
        );

        generator.generate_voxels(offset, &mut self.voxels, CHUNK_VOXELS_SIZE)
    }

    pub fn generate(&mut self, pos: PosComponent, generator: &GeneratorRes) {
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

    pub fn get_chunk_pos(pos: PosComponent) -> PosComponent {
        PosComponent::new(
            Self::cord_to_chunk_cord(pos.x),
            Self::cord_to_chunk_cord(pos.y),
            Self::cord_to_chunk_cord(pos.z),
        )
    }

    pub fn get_chunk_pos_by_vec(vec: Vec3) -> PosComponent {
        Self::get_chunk_pos(PosComponent::new(vec.x as i64, vec.y as i64, vec.z as i64))
    }

    pub fn get_chunk_pos_by_transform(transform: &Transform) -> PosComponent {
        Self::get_chunk_pos_by_vec(transform.translation)
    }

    pub fn get_voxel(&self, in_chunk_position: PosComponent) -> Option<Voxel> {
        match Self::pos_to_index(in_chunk_position.clone()) {
            Some(index) => {
                return Some(self.voxels[index]);
            }
            _ => None,
        }
    }

    pub fn pos_to_relative(chunk_pos: PosComponent, pos: PosComponent) -> PosComponent {
        pos - chunk_pos.mul_scalar(CHUNK_REAL_SIZE as i64)
    }

    pub fn index_to_pos(index: usize) -> PosComponent {
        return PosComponent {
            x: (index % CHUNK_VOXELS_SIZE) as i64,
            y: ((index / CHUNK_VOXELS_SIZE) % CHUNK_VOXELS_SIZE) as i64,
            z: (index / CHUNK_VOXELS_SIZE / CHUNK_VOXELS_SIZE) as i64,
        };
    }

    pub fn dig(
        &mut self,
        chunk_pos: PosComponent,
        voxel_pos: PosComponent,
        radius: f32,
        value: f32,
    ) -> usize {
        let mut count: usize = 0;

        for i in 0..CHUNK_VOXELS_VOLUME {
            let delta_pos = Chunk::pos_to_relative(chunk_pos, voxel_pos) - Self::index_to_pos(i);

            let delta_vec = Vec3::new(delta_pos.x as f32, delta_pos.y as f32, delta_pos.z as f32);
            let l = delta_vec.length();

            if l < radius {
                count += 1;
                self.voxels[i].value -= value * (radius - l) / radius / (self.voxels[i].value + 1.);
                self.voxels[i].value = self.voxels[i].value.max(-0.001);
            }
        }

        return count;
    }

    pub fn fill(
        &mut self,
        generator: &GeneratorRes,
        chunk_pos: PosComponent,
        voxel_pos: PosComponent,
        voxel: Voxel,
        radius: f32,
        value: f32,
    ) -> usize {
        let mut count: usize = 0;

        for i in 0..CHUNK_VOXELS_VOLUME {
            let pos = Self::index_to_pos(i);
            let delta_pos = Chunk::pos_to_relative(chunk_pos, voxel_pos) - pos;

            let delta_vec = Vec3::new(delta_pos.x as f32, delta_pos.y as f32, delta_pos.z as f32);
            let l = delta_vec.length();

            if l < radius {
                count += 1;
                self.voxels[i].color =
                    generator
                        .noise
                        .randomize_color(pos.to_vec(), voxel.color, 0.2);
                self.voxels[i].value = self.voxels[i].value.max(-0.001);
                self.voxels[i].value += value * (radius - l) / radius / (self.voxels[i].value + 1.);
                self.voxels[i].value = self.voxels[i].value.min(1.);
            }
        }

        return count;
    }
}
