use bevy::{math::Vec3, prelude::*};

use crate::plugins::generator::resources::GeneratorRes;

use super::BiomeGenerator;

pub struct Cave2dLayer {
    stalactites_val: f32,
    moss_val: f32,
}

pub struct CaveBiomeGenerator;

impl BiomeGenerator<Cave2dLayer> for CaveBiomeGenerator {
    fn get_2d_layer(&self, generator: GeneratorRes, pos: Vec2) -> Cave2dLayer {
        let stalactites_val = self.get_stalactites_val(&generator, pos);
        let moss_val = self.get_moss_area(&generator, pos);
        Cave2dLayer {
            stalactites_val,
            moss_val,
        }
    }

    fn get_voxel_color(
        &self,
        generator: GeneratorRes,
        layer2d: &Cave2dLayer,
        pos: Vec3,
        value: f32,
    ) -> Color {
        let mut color = Color::rgb(0.3, 0.3, 0.4);
        if layer2d.stalactites_val < 0.1 && value < 0.001 {
            color = Color::rgb(
                0.3,
                0.3 + layer2d.moss_val * 0.3,
                0.4 + layer2d.moss_val * 0.6,
            );
        }

        color
    }

    fn get_voxel_value(&self, generator: GeneratorRes, layer2d: &Cave2dLayer, pos: Vec3) -> f32 {
        let mut noise_v = self.get_cliffs_val(&generator, pos);
        noise_v += self.get_cylinder_val(&generator, pos);
        noise_v += layer2d.stalactites_val;
        noise_v /= 100.;
        noise_v = noise_v.min(0.1);

        noise_v
    }
}

impl CaveBiomeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_cliffs_val(&self, generator: &GeneratorRes, pos: Vec3) -> f32 {
        let mut val = generator.get_noise3(pos);

        val -= generator.get_noise3(pos * 0.156) * 0.7;
        val *= 0.7;

        val = val.min(1.);

        val
    }

    fn get_cylinder_val(&self, generator: &GeneratorRes, pos: Vec3) -> f32 {
        let r = 0.8 + generator.get_norm_noise(pos.z * 0.1) * 0.6;
        let z = pos.z / 2. + generator.get_noise(pos.z / 4.);

        let x_shift = generator.get_noise(pos.z * 0.05) * 5.;
        let shifted_pos = Vec2::new(pos.x * 0.5 + x_shift, (pos.y + z) * 2.);
        let mut val = (shifted_pos / r).length() - r;

        val *= 1.5;
        val = val.min(1.);

        val
    }

    fn get_stalactites_val(&self, generator: &GeneratorRes, pos: Vec2) -> f32 {
        let mut density = generator.get_norm_noise2(pos * 2.);
        density *= density;

        let mut val = generator.get_norm_noise2(pos * 10.) * density;
        val *= val * 10.;

        val = val.min(1.);

        val
    }

    fn get_moss_area(&self, generator: &GeneratorRes, pos: Vec2) -> f32 {
        let big_areas = generator.get_noise(pos.y * 0.1);
        let small_areas = generator.get_noise2(pos * 10.);

        (big_areas + small_areas).max(0.).min(1.)
    }
}
