use bevy::{math::Vec3, prelude::*};

use crate::plugins::{
    chunk::resources::chunk::object::Object, generator::resources::noise_generator::NoiseGenerator,
};

use super::BiomeGenerator;

#[derive(Default)]
pub struct Cave2dLayer {
    stalactites_val: f32,
    moss_val: f32,
}

pub struct CaveBiomeGenerator {
    layer2d: Cave2dLayer,
}

impl BiomeGenerator for CaveBiomeGenerator {
    fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2) {
        let stalactites_val = self.get_stalactites_val(&generator, pos);
        let moss_val = self.get_moss_area(&generator, pos);
        self.layer2d = Cave2dLayer {
            stalactites_val,
            moss_val,
        };
    }

    fn get_voxel_color(&self, _generator: &NoiseGenerator, _pos: Vec3, value: f32) -> Color {
        let mut color = Color::rgb(0.3, 0.3, 0.4);
        if self.layer2d.stalactites_val < 0.1 && value < 0.001 {
            color = Color::rgb(
                0.3,
                0.3 + self.layer2d.moss_val * 0.3,
                0.4 + self.layer2d.moss_val * 0.6,
            );
        }

        color
    }

    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let mut noise_v = self.get_cliffs_val(&generator, pos);
        noise_v += self.get_cylinder_val(&generator, pos);
        noise_v += self.layer2d.stalactites_val;
        noise_v /= 100.;
        noise_v = noise_v.min(0.1);

        noise_v
    }

    fn try_generate_object(&self, _pos: Vec3, _value: f32) -> Option<Object> {
        None
    }
}

impl CaveBiomeGenerator {
    pub fn new() -> Self {
        Self {
            layer2d: Cave2dLayer::default(),
        }
    }

    fn get_cliffs_val(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let mut val = generator.get_noise3(pos);

        val -= generator.get_noise3(pos * 0.156) * 0.7;
        val *= 0.7;

        val = val.min(1.);

        val
    }

    fn get_cylinder_val(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let r = 0.8 + generator.get_norm_noise(pos.z * 0.1) * 0.6;

        let x_shift = generator.get_noise(pos.z * 0.05) * 5.;
        let shifted_pos = Vec2::new(pos.x * 0.5 + x_shift, pos.y * 2.);
        let mut val = (shifted_pos / r).length() - r;

        val *= 1.5;
        val = val.min(1.);

        val
    }

    fn get_stalactites_val(&self, generator: &NoiseGenerator, pos: Vec2) -> f32 {
        let mut density = generator.get_norm_noise2(pos * 2.);
        density *= density;

        let mut val = generator.get_norm_noise2(pos * 10.) * density;
        val *= val * 10.;

        val = val.min(1.);

        val
    }

    fn get_moss_area(&self, generator: &NoiseGenerator, pos: Vec2) -> f32 {
        let big_areas = generator.get_noise(pos.y * 0.1);
        let small_areas = generator.get_noise2(pos * 10.);

        (big_areas + small_areas).max(0.).min(1.)
    }
}
