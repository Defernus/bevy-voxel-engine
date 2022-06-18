use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use voxel_engine_bevy::plugins::{
    chunk::ChunkPlugin, generator::GeneratorPlugin, player::PlayerPlugin, window::WindowPlugin,
    world::WorldPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WindowPlugin)
        .add_plugin(GeneratorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(ChunkPlugin)
        .run();
}
