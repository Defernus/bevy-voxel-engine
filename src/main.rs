use bevy::prelude::*;
use voxel_engine_bevy::plugins::{
    camera::CameraPlugin, chunk::ChunkPlugin, generator::GeneratorPlugin, world::WorldPlugin,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(GeneratorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(ChunkPlugin)
        .run();
}
