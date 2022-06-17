use bevy::prelude::*;

use self::resources::GeneratorRes;

pub mod resources;
pub struct GeneratorPlugin;

fn generator_startup_system() {}

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeneratorRes::default())
            .add_startup_system(generator_startup_system);
    }
}
