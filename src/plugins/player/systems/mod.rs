use crate::plugins::player::components::PlayerComponent;
use bevy::prelude::*;

use super::components::PlayerLightComponent;

pub mod control_system;
pub mod move_system;
pub mod rotate_system;

pub fn player_startup_system(mut commands: Commands) {
    commands.spawn((PlayerComponent::default(), Transform::default()));
    commands.spawn((
        PlayerLightComponent,
        PointLightBundle {
            transform: Transform::default(),
            point_light: PointLight {
                intensity: 1_000_000.,
                range: 500.,
                color: Color::srgb(1., 0.9, 0.7),
                shadows_enabled: false,
                ..default()
            },
            ..default()
        },
    ));
}
