use crate::plugins::player::components::PlayerComponent;
use bevy::prelude::*;

pub mod move_system;
pub mod rotate_system;

pub fn player_startup_system(mut commands: Commands) {
    commands
        .spawn()
        .insert(PlayerComponent)
        .insert(Transform::from_xyz(0., 0., 0.).looking_at(-Vec3::Z, Vec3::Y));
}
