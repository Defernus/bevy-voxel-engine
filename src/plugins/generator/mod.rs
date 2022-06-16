use bevy::prelude::*;

use crate::resources::generator::GeneratorResource;

pub struct GeneratorPlugin;

fn generator_startup_system() {}

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratorResource::default())
            .add_startup_system(generator_startup_system);
    }
}
