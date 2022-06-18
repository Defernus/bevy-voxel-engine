use bevy::prelude::*;

use crate::plugins::{
    camera::components::{update_camera_transform, CameraComponent},
    player::components::PlayerComponent,
};

pub fn player_move_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<CameraComponent>, Without<PlayerComponent>)>,
    mut player: Query<&mut Transform, (With<PlayerComponent>, Without<CameraComponent>)>,
) {
    let mut camera = camera.get_single_mut().expect("camera does not exists yet");
    let camera = camera.as_mut();

    let mut player = player.get_single_mut().expect("player does not exists yet");
    let imut_player = player.clone();

    let dt = time.delta_seconds_f64() as f32;
    let speed = if keys.pressed(KeyCode::LShift) {
        50.
    } else {
        10.
    };

    if keys.pressed(KeyCode::W) {
        player.translation += imut_player.forward() * speed * dt;
    }

    if keys.pressed(KeyCode::S) {
        player.translation += imut_player.back() * speed * dt;
    }

    if keys.pressed(KeyCode::A) {
        player.translation += imut_player.left() * speed * dt;
    }

    if keys.pressed(KeyCode::D) {
        player.translation += imut_player.right() * speed * dt;
    }

    if keys.pressed(KeyCode::LControl) {
        player.translation -= Vec3::Y * speed * dt;
    }

    if keys.pressed(KeyCode::Space) {
        player.translation += Vec3::Y * speed * dt;
    }

    update_camera_transform(imut_player, camera);
}
