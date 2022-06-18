use bevy::prelude::*;

use crate::plugins::{camera::components::CameraComponent, player::components::PlayerComponent};

pub fn camera_follow_system(
    player_q: Query<&Transform, (With<PlayerComponent>, Without<CameraComponent>)>,
    mut camera_q: Query<&mut Transform, (With<CameraComponent>, Without<PlayerComponent>)>,
) {
    let player_transform = player_q.get_single().expect("player does not exists yet");
    let mut cam_transform = camera_q
        .get_single_mut()
        .expect("camera does not exists yet");

    cam_transform.scale = player_transform.scale;
    cam_transform.rotation = player_transform.rotation;
    cam_transform.translation = player_transform.translation;
}
