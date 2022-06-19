use bevy::prelude::*;

use crate::plugins::{
    camera::components::{update_camera_transform, CameraComponent},
    player::components::PlayerComponent,
};

mod move_glide;
mod move_noclip;

pub fn player_move_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera_q: Query<&mut Transform, (With<CameraComponent>, Without<PlayerComponent>)>,
    mut player_q: Query<(&mut Transform, &mut PlayerComponent), Without<CameraComponent>>,
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

    update_camera_transform(player_transform.clone(), camera_transform);
}
