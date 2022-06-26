use bevy::{
    math::{Vec2, Vec3},
    prelude::Color,
};

use crate::plugins::chunk::resources::chunk::{object::Object, voxel::Voxel};

use self::{cave::CaveBiomeGenerator, glade::GladeBiomeGenerator};

use super::noise_generator::NoiseGenerator;

pub mod cave;
pub mod glade;

pub trait BiomeGenerator {
    fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2);
    fn get_voxel_color(&self, generator: &NoiseGenerator, pos: Vec3, value: f32) -> Color;
    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32;
    fn try_generate_object(&self, pos: Vec3, value: f32) -> Option<Object>;
}

pub struct BiomesHandler {
    prev_biome: Box<dyn BiomeGenerator>,
    next_biome: Box<dyn BiomeGenerator>,
    pub voxel: Voxel,

    prev_value: f32,
    next_value: f32,

    transition: f32,
}

const BIOMES_COUNT: usize = 2;

fn get_biome_by_id(id: usize) -> Box<dyn BiomeGenerator> {
    match id % BIOMES_COUNT {
        0 => Box::new(CaveBiomeGenerator::new()),
        1 => Box::new(GladeBiomeGenerator::new()),
        _ => {
            unreachable!("id({}) should be lower then {}", id, BIOMES_COUNT);
        }
    }
}

impl BiomesHandler {
    pub fn new(generator: &NoiseGenerator, pos: f32, transition_gap: f32) -> Self {
        let pos = pos / 10.;

        let prev_id = generator.get_norm_noise(pos.floor());
        let next_id = generator.get_norm_noise(pos.ceil());

        let prev_id = (prev_id * 1000000.) as usize % BIOMES_COUNT;
        let next_id = (next_id * 1000000.) as usize % BIOMES_COUNT;

        let transition = (pos - pos.floor()) - 0.5;
        let transition = transition / transition_gap + 0.5;

        Self {
            voxel: Voxel {
                value: 0.,
                color: Color::BLACK,
            },
            prev_value: 0.,
            next_value: 0.,
            transition: transition.min(1.).max(0.),
            prev_biome: get_biome_by_id(prev_id),
            next_biome: get_biome_by_id(next_id),
        }
    }

    pub fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2) {
        if self.transition < 1. {
            self.prev_biome.update_2d_layer(generator, pos);
        }
        if self.transition > 0. {
            self.next_biome.update_2d_layer(generator, pos);
        }
    }

    pub fn update_3d(&mut self, generator: &NoiseGenerator, mut pos: Vec3) {
        pos.y += pos.z / 2. + generator.get_noise(pos.z / 4.);

        self.prev_value = if self.transition < 1. {
            self.prev_biome.get_voxel_value(generator, pos)
        } else {
            0.
        };

        self.next_value = if self.transition > 0. {
            self.next_biome.get_voxel_value(generator, pos)
        } else {
            0.
        };

        let prev_color = if self.transition < 1. {
            self.prev_biome
                .get_voxel_color(generator, pos, self.prev_value)
        } else {
            Color::BLACK
        };
        let next_color = if self.transition > 0. {
            self.next_biome
                .get_voxel_color(generator, pos, self.next_value)
        } else {
            Color::BLACK
        };

        self.voxel = Voxel {
            value: self.prev_value * (1. - self.transition) + self.next_value * self.transition,
            color: prev_color * (1. - self.transition) + next_color * self.transition,
        };
    }

    pub fn try_generate_object(
        &self,
        generator: &NoiseGenerator,
        mut pos: Vec3,
        value: f32,
    ) -> Option<Object> {
        pos.y += pos.z / 2. + generator.get_noise(pos.z / 4.);

        if self.transition < 0.5 {
            self.prev_biome.try_generate_object(pos, value)
        } else {
            self.prev_biome.try_generate_object(pos, value)
        }
    }
}
