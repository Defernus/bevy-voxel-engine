use crate::plugins::player::components::PlayerComponent;
use bevy::{prelude::*, render::camera::Projection};
use bevy_mod_raycast::RayCastSource;

use crate::common::components::ray_let::RayLet;

pub mod components;
mod systems;
pub struct PlayerPlugin;

fn player_startup_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 2.0,
                ..default()
            }),
            ..default()
        })
        .insert(RayCastSource::<RayLet>::new_transform_empty())
        .insert(PlayerComponent);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_startup_system)
            .add_system(systems::move_system::player_move_system)
            .add_system(systems::rotate_system::player_rotate_system);
    }
}
