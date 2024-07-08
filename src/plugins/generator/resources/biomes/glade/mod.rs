use bevy::{math::Vec3, prelude::*};

use crate::plugins::{
    chunk::resources::chunk::object::Object, generator::resources::noise_generator::NoiseGenerator,
};

use super::BiomeGenerator;

#[derive(Default)]
pub struct Glade2dLayer {
    grass_value: f32,
}

pub struct GladeBiomeGenerator {
    layer2d: Glade2dLayer,
}

impl BiomeGenerator for GladeBiomeGenerator {
    fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2) {
        let grass_value = generator.get_noise2(pos * 100.);

        self.layer2d = Glade2dLayer { grass_value }
    }

    fn get_voxel_color(&self, _generator: &NoiseGenerator, pos: Vec3, value: f32) -> Color {
        let mut color = Color::srgb(0.3, 0.3, 0.4);

        if pos.y + 0.5 - self.layer2d.grass_value < 0. && value < 0.003 {
            color = Color::srgb(0.1, 1.0, 0.3);
        }

        color
    }

    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let mut noise_v = self.get_cliffs_val(&generator, pos);
        noise_v += self.get_cylinder_val(&generator, pos);
        noise_v /= 100.;
        noise_v = noise_v.min(0.1);

        noise_v
    }

    fn try_generate_object(&self, pos: Vec3, value: f32) -> Option<Object> {
        if pos.y > 1. && value.abs() < 0.0001 {
            Some(Object::new(0))
        } else {
            None
        }
    }
}

impl GladeBiomeGenerator {
    pub fn new() -> Self {
        Self {
            layer2d: Glade2dLayer::default(),
        }
    }

    fn get_cliffs_val(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let mut val = generator.get_noise3(pos);
        val += generator.get_noise3(pos * 10.) * 0.1;

        val -= generator.get_noise3(pos * 0.156) * 2.0;
        val *= 0.7;

        val = val.min(1.);

        val
    }

    fn get_cylinder_val(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let r = 1. + generator.get_norm_noise(pos.z * 0.1) * 0.6;

        let x_shift = generator.get_noise(pos.z * 0.05) * 5.;
        let shifted_pos = Vec2::new(pos.x * 0.5 + x_shift, pos.y * 1.);
        let mut val = (shifted_pos / r).length() - r;

        val *= 1.5;
        val = val.min(1.);

        val
    }
}
