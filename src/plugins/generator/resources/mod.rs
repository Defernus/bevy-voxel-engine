use bevy::math::{Vec2, Vec3};

use crate::plugins::chunk::resources::chunk::voxel::Voxel;

use self::{biomes::BiomesHandler, noise_generator::NoiseGenerator};

mod biomes;
mod noise_generator;

#[derive(Clone)]
pub struct GeneratorRes {
    scale: f32,
    pub noise: NoiseGenerator,
}

impl GeneratorRes {
    pub fn generate_voxels(&self, offset: Vec3, voxels: &mut [Voxel], size: usize) {
        for z in 0..size {
            let mut biomes_handler =
                BiomesHandler::new(&self.noise, (offset.z + z as f32) * self.scale, 0.5);

            for x in 0..size {
                let pos2 = Vec2::new(offset.x + x as f32, offset.z + z as f32) * self.scale;
                biomes_handler.update_2d_layer(&self.noise, pos2);

                for y in 0..size {
                    let pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    ) * self.scale;

                    let mut voxel = biomes_handler.get_voxel(&self.noise, pos);
                    voxel.color = self.noise.randomize_color(pos, voxel.color, 0.2);

                    voxels[x + y * size + z * size * size] = voxel;
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
