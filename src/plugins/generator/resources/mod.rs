use bevy::{math::Vec3, prelude::Color};
use noise::NoiseFn;

use crate::plugins::chunk::resources::chunk::voxel::Voxel;

#[derive(Clone)]
pub struct GeneratorRes {
    simplex: noise::OpenSimplex,
    scale: f64,
    noise_threshold: f64,
}

impl GeneratorRes {
    fn get_level_val(&self, pos: Vec3) -> f64 {
        let mut noise_v: f64 = (pos.y as f64)
            + (self.simplex.get([
                (pos.x as f64) * 0.456 * self.scale,
                (pos.z as f64) * 0.456 * self.scale,
            ]) + 1.)
                * 10.
            + (self.simplex.get([pos.x as f64, pos.z as f64]) + 1.) * 0.001;

        noise_v -= self.noise_threshold;
        noise_v /= 100. / self.scale;

        return -noise_v;
    }

    fn get_cliffs_val(&self, pos: Vec3) -> f64 {
        let mut noise_v = self.simplex.get([
            (pos.x as f64) * self.scale,
            (pos.y as f64) * self.scale,
            (pos.z as f64) * self.scale,
        ]);
        noise_v += 1.0;
        noise_v /= 2.0;

        noise_v -= (self.simplex.get([
            (pos.x as f64) * 0.156 * self.scale,
            (pos.y as f64) * 0.156 * self.scale,
            (pos.z as f64) * 0.156 * self.scale,
        ]) + 1.)
            / 2.
            * self.noise_threshold;
        noise_v *= 10. * self.scale;

        return noise_v;
    }

    fn randomize_color(&self, pos: Vec3, color: Color, factor: f64) -> Color {
        let dr = self.simplex.get([
            (pos.x as f64) / 2.3,
            (pos.y as f64) / 2.3,
            (pos.z as f64) / 2.3,
        ]) * factor;
        let dg = self.simplex.get([
            -(pos.x as f64) / 2.3,
            (pos.y as f64) / 2.3,
            (pos.z as f64) / 2.3,
        ]) * factor;
        let db = self.simplex.get([
            (pos.x as f64) / 2.3,
            -(pos.y as f64) / 2.3,
            (pos.z as f64) / 2.3,
        ]) * factor;
        Color::rgb(
            (color.r() + dr as f32).max(0.).min(1.),
            (color.g() + dg as f32).max(0.).min(1.),
            (color.b() + db as f32).max(0.).min(1.),
        )
    }

    pub fn generate_voxels(&self, offset: Vec3, voxels: &mut [Voxel], size: usize) {
        for x in 0..size {
            for z in 0..size {
                for y in 0..size {
                    let pos = Vec3::new(
                        offset.x + x as f32,
                        offset.y + y as f32,
                        offset.z + z as f32,
                    );

                    let level = self.get_level_val(pos);

                    let mut color = Color::rgb(0.4, 0.4, 0.4);

                    if level < 0.03 * self.scale {
                        color = Color::rgb(0.2, 0.7, 0.3);
                    }

                    let mut noise_v = self.get_cliffs_val(pos);

                    noise_v = noise_v.min(level);

                    voxels[x + y * size + z * size * size] = Voxel {
                        color: self.randomize_color(pos, color, 0.05),
                        value: noise_v.max(-0.1).min(1.) as f32,
                    };
                }
            }
        }
    }
}

impl Default for GeneratorRes {
    fn default() -> Self {
        Self {
            simplex: noise::OpenSimplex::new(),
            scale: 0.1,
            noise_threshold: 0.6,
        }
    }
}
