use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
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
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Mailbox,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.28, 0.27, 0.31)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultRaycastingPlugin::<RayLet>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WindowPlugin)
        .add_plugin(GeneratorPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(ChunkPlugin)
        .run();
}
