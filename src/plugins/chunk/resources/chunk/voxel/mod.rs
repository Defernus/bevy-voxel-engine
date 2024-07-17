use bevy::prelude::Color;

pub mod voxels_to_vertex;
#[allow(dead_code)]
pub struct VoxelId(u32);

#[derive(Clone, Copy)]
pub struct Voxel {
    pub value: f32,
    pub color: Color,
}
