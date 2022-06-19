use bevy::{
    math::{Vec2, Vec3},
    prelude::Color,
};
use noise::NoiseFn;

use crate::plugins::chunk::resources::chunk::voxel::Voxel;

#[derive(Clone)]
pub struct GeneratorRes {
    simplex: noise::OpenSimplex,
    perlin: noise::Perlin,
    scale: f32,
    noise_threshold: f32,
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

    fn get_cliffs_val(&self, pos: Vec3) -> f32 {
        let mut val = self.get_noise3(pos);

        val -= self.get_noise3(pos * 0.156) * self.noise_threshold;
        val *= 0.7;

        val = val.min(1.);

        val
    }

    fn get_cylinder_val(&self, pos: Vec3) -> f32 {
        let r = 0.8 + self.get_norm_noise(pos.z * 0.1) * 0.6;
        let z = pos.z / 2. + self.get_noise(pos.z / 4.);

        let x_shift = self.get_noise(pos.z * 0.05) * 5.;
        let shifted_pos = Vec2::new(pos.x * 0.5 + x_shift, (pos.y + z) * 2.);
        let mut val = (shifted_pos / r).length() - r;

        val *= 1.5;
        val = val.min(1.);

        val
    }

    fn get_stalactites_val(&self, pos: Vec2) -> f32 {
        let mut density = self.get_norm_noise2(pos * 2.);
        density *= density;

        let mut val = self.get_norm_noise2(pos * 10.) * density;
        val *= val * 10.;

        val = val.min(1.);

        val
    }

    fn get_moss_area(&self, pos: Vec2) -> f32 {
        let big_areas = self.get_noise2(pos * 0.1);
        let small_areas = self.get_noise2(pos * 10.);

        (big_areas + small_areas).max(0.).min(1.)
    }

    pub fn generate_voxels(&self, offset: Vec3, voxels: &mut [Voxel], size: usize) {
        for x in 0..size {
            for z in 0..size {
                let pos2 = Vec2::new(offset.x + x as f32, offset.z + z as f32) * self.scale;
                let stalactites_val = self.get_stalactites_val(pos2);
                let moss_val = self.get_moss_area(pos2);

                for y in 0..size {
                    let pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    ) * self.scale;

                    let mut color = Color::rgb(0.3, 0.3, 0.4);

                    let mut noise_v = self.get_cliffs_val(pos);
                    noise_v += self.get_cylinder_val(pos);
                    noise_v += stalactites_val;

                    if stalactites_val < 0.1 && noise_v < 0.1 {
                        color = Color::rgb(0.3, 0.3 + moss_val * 0.3, 0.4 + moss_val * 0.6);
                    }

                    noise_v /= 100.;
                    noise_v = noise_v.min(0.1);

                    voxels[x + y * size + z * size * size] = Voxel {
                        color: self.randomize_color(pos, color, 0.2),
                        value: noise_v as f32,
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
            noise_threshold: 0.7,
            perlin: noise::Perlin::default(),
            simplex: noise::OpenSimplex::default(),
        }
    }
}
