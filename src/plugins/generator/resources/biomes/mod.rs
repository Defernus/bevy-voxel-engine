use bevy::{
    math::{Vec2, Vec3},
    prelude::Color,
};

use super::noise_generator::NoiseGenerator;

pub mod cave;

pub trait BiomeGenerator<T> {
    fn get_2d_layer(&self, generator: &NoiseGenerator, pos: Vec2) -> T;
    fn get_voxel_color(
        &self,
        generator: &NoiseGenerator,
        layer2d: &T,
        pos: Vec3,
        value: f32,
    ) -> Color;
    fn get_voxel_value(&self, generator: &NoiseGenerator, layer2d: &T, pos: Vec3) -> f32;
}
