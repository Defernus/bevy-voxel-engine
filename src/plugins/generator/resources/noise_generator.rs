use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
};
use noise::NoiseFn;

#[derive(Clone)]
pub struct NoiseGenerator {
    simplex: noise::OpenSimplex,
    perlin: noise::Perlin,
}

impl NoiseGenerator {
    pub fn new() -> Self {
        Self {
            perlin: noise::Perlin::default(),
            simplex: noise::OpenSimplex::default(),
        }
    }

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
        let dr = self.get_noise3(pos / 2.4) * factor;
        let dg = self.get_noise3(pos / 2.4 * Vec3::new(-1., 1., 1.)) * factor;
        let db = self.get_noise3(pos / 2.4 * Vec3::new(1., -1., 1.)) * factor;

        Color::rgb(
            (color.r() + color.r() * dr).max(0.).min(1.),
            (color.g() + color.g() * dg).max(0.).min(1.),
            (color.b() + color.b() * db).max(0.).min(1.),
        )
    }
}
