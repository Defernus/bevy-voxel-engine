use bevy::math::{Vec2, Vec3};

use crate::plugins::chunk::resources::chunk::voxel::Voxel;

use self::{
    biomes::{cave::CaveBiomeGenerator, BiomeGenerator},
    noise_generator::NoiseGenerator,
};

mod biomes;
mod noise_generator;

#[derive(Clone)]
pub struct GeneratorRes {
    scale: f32,
    pub noise: NoiseGenerator,
}

impl GeneratorRes {
    pub fn generate_voxels(&self, offset: Vec3, voxels: &mut [Voxel], size: usize) {
        let biome = CaveBiomeGenerator::new();
        for x in 0..size {
            for z in 0..size {
                let pos2 = Vec2::new(offset.x + x as f32, offset.z + z as f32) * self.scale;
                let layer = biome.get_2d_layer(&self.noise, pos2);

                for y in 0..size {
                    let pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    ) * self.scale;

                    let value = biome.get_voxel_value(&self.noise, &layer, pos);
                    let color = biome.get_voxel_color(&self.noise, &layer, pos, value);

                    voxels[x + y * size + z * size * size] = Voxel {
                        color: self.noise.randomize_color(pos, color, 0.2),
                        value,
                    };
                }
            }
        }
    }
}

impl Default for GeneratorRes {
    fn default() -> Self {
        Self {
            scale: 0.1,
            noise: NoiseGenerator::new(),
        }
    }
}
