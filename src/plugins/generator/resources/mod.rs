use bevy::math::{Vec2, Vec3};
use noise::NoiseFn;

use crate::plugins::chunk::resources::chunk::{object::Object, voxel::Voxel};

use self::{biomes::BiomesHandler, noise_generator::NoiseGenerator};

mod biomes;
mod noise_generator;

#[derive(Clone)]
pub struct GeneratorRes {
    scale: f32,
    object_spawn_tries: usize,
    pub noise: NoiseGenerator,
}

impl GeneratorRes {
    pub fn generate_voxels(
        &self,
        offset: Vec3,
        voxels: &mut [Voxel],
        objects: &mut Vec<Object>,
        size: usize,
    ) {
        for z in 0..size {
            let mut biomes_handler =
                BiomesHandler::new(&self.noise, (offset.z + z as f32) * self.scale, 0.5);

            for x in 0..size {
                let pos2 = Vec2::new(offset.x + x as f32, offset.z + z as f32) * self.scale;
                biomes_handler.update_2d_layer(&self.noise, pos2);

                for y in 0..size {
                    let unscaled_pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    );
                    let pos = unscaled_pos * self.scale;

                    biomes_handler.update_3d(&self.noise, pos);

                    let mut voxel = biomes_handler.voxel;
                    voxel.color = self.noise.randomize_color(pos, voxel.color, 0.2);

                    voxels[x + y * size + z * size * size] = voxel;
                }
            }

            self.generate_objects(voxels, z, offset, &biomes_handler, objects, size);
        }
    }

    fn generate_objects(
        &self,
        voxels: &[Voxel],
        z: usize,
        offset: Vec3,
        biomes_handler: &BiomesHandler,
        objects: &mut Vec<Object>,
        size: usize,
    ) -> usize {
        let mut generated = 0;
        for i in 0..self.object_spawn_tries {
            let pos_arr = [0., 1.].map(|w| {
                (self.noise.simplex.get([
                    offset.x as f64,
                    offset.y as f64,
                    offset.z as f64 + z as f64,
                    w + (i * 2) as f64,
                ]) as f32
                    * 0.5
                    + 0.5)
                    * size as f32
            });

            let voxel_pos = Vec3::new(pos_arr[0], pos_arr[1], z as f32);

            let unscaled_pos = voxel_pos + offset;

            let pos = unscaled_pos * self.scale;

            let voxel =
                voxels[voxel_pos.x as usize + voxel_pos.y as usize * size + z * size * size];

            match biomes_handler.try_generate_object(&self.noise, pos, voxel.value) {
                Some(mut object) => {
                    generated += 1;
                    object.transform.translation = unscaled_pos;
                    objects.push(object);
                }
                _ => {}
            };
        }

        return generated;
    }
}

impl Default for GeneratorRes {
    fn default() -> Self {
        Self {
            object_spawn_tries: 5,
            scale: 0.1,
            noise: NoiseGenerator::new(),
        }
    }
}
