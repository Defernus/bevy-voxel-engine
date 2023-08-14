use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_mod_raycast::DefaultRaycastingPlugin;
use voxel_engine_bevy::{
    common::components::ray_let::RayLet,
    plugins::{
        camera::CameraPlugin, chunk::ChunkPlugin, generator::GeneratorPlugin, player::PlayerPlugin,
        window::WindowPlugin, world::WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultRaycastingPlugin::<RayLet>::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WindowPlugin)
        .add_plugins(GeneratorPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(ChunkPlugin)
        .run();
}
