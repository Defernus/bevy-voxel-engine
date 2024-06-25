use bevy::prelude::*;

use crate::plugins::{
    camera::components::{update_camera_transform, CameraComponent},
    player::components::{PlayerComponent, PlayerLightComponent},
};

mod move_glide;
mod move_noclip;

pub fn player_move_system(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut light_q: Query<
        &mut Transform,
        (
            With<PlayerLightComponent>,
            Without<PlayerComponent>,
            Without<CameraComponent>,
        ),
    >,
    mut camera_q: Query<
        &mut Transform,
        (
            With<CameraComponent>,
            Without<PlayerComponent>,
            Without<PlayerLightComponent>,
        ),
    >,
    mut player_q: Query<
        (&mut Transform, &mut PlayerComponent),
        (Without<CameraComponent>, Without<PlayerLightComponent>),
    >,
) {
    let mut camera = camera_q
        .get_single_mut()
        .expect("camera does not exists yet");
    let camera_transform = camera.as_mut();

    let (mut player_transform, mut player) = player_q
        .get_single_mut()
        .expect("player does not exists yet");
    let dt = time.delta_seconds_f64() as f32;

    if player.noclip {
        move_noclip::move_noclip(&keys, player.as_mut(), player_transform.as_mut(), dt);
    } else {
        move_glide::move_glide(&keys, player.as_mut(), player_transform.as_mut(), dt);
    }

    let mut light_transform = light_q
        .get_single_mut()
        .expect("player's light does not exists yet");

    *light_transform = player_transform.clone();

    update_camera_transform(player_transform.clone(), camera_transform);
}
