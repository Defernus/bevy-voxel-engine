use bevy::{
    math::{Vec2, Vec3},
    prelude::Color,
};
use noise::NoiseFn;

use crate::plugins::chunk::resources::chunk::voxel::Voxel;

use self::biomes::{cave::CaveBiomeGenerator, BiomeGenerator};

mod biomes;

#[derive(Clone)]
pub struct GeneratorRes {
    simplex: noise::OpenSimplex,
    perlin: noise::Perlin,
    scale: f32,
}

impl GeneratorRes {
    pub fn get_noise(&self, pos: f32) -> f32 {
        self.perlin.get([pos as f64, 0.]) as f32
    }

    pub fn get_norm_noise(&self, pos: f32) -> f32 {
        (self.get_noise(pos) + 1.) / 2.
    }

    pub fn get_noise2(&self, pos: Vec2) -> f32 {
        self.simplex.get([pos.x as f64, pos.y as f64]) as f32
    }

    pub fn get_norm_noise2(&self, pos: Vec2) -> f32 {
        (self.get_noise2(pos) + 1.) / 2.
    }

    pub fn get_noise3(&self, pos: Vec3) -> f32 {
        self.simplex.get([pos.x as f64, pos.y as f64, pos.z as f64]) as f32
    }

    pub fn get_norm_noise3(&self, pos: Vec3) -> f32 {
        (self.get_noise3(pos) + 1.) / 2.
    }

    pub fn randomize_color(&self, pos: Vec3, color: Color, factor: f32) -> Color {
        let pos = pos * self.scale;

        let dr = self.get_noise3(pos / 2.4) * factor;
        let dg = self.get_noise3(pos / 2.4 * Vec3::new(-1., 1., 1.)) * factor;
        let db = self.get_noise3(pos / 2.4 * Vec3::new(1., -1., 1.)) * factor;

        Color::rgb(
            (color.r() + color.r() * dr).max(0.).min(1.),
            (color.g() + color.g() * dg).max(0.).min(1.),
            (color.b() + color.b() * db).max(0.).min(1.),
        )
    }

    pub fn generate_voxels(&self, offset: Vec3, voxels: &mut [Voxel], size: usize) {
        let biome = CaveBiomeGenerator::new();
        for x in 0..size {
            for z in 0..size {
                let pos2 = Vec2::new(offset.x + x as f32, offset.z + z as f32) * self.scale;
                let layer = biome.get_2d_layer(self.clone(), pos2);

                for y in 0..size {
                    let pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    ) * self.scale;

                    let value = biome.get_voxel_value(self.clone(), &layer, pos);
                    let color = biome.get_voxel_color(self.clone(), &layer, pos, value);

                    voxels[x + y * size + z * size * size] = Voxel {
                        color: self.randomize_color(pos, color, 0.2),
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
            perlin: noise::Perlin::default(),
            simplex: noise::OpenSimplex::default(),
        }
    }
}
