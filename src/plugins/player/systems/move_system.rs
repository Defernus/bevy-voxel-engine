use bevy::prelude::*;

use crate::plugins::player::components::PlayerComponent;

pub fn player_move_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<PlayerComponent>>,
) {
    let mut cam = player.get_single_mut().expect("player does not exists yet");
    let imut_cam = cam.clone();

    let dt = time.delta_seconds_f64() as f32;
    let speed = if keys.pressed(KeyCode::LShift) {
        50.
    } else {
        10.
    };

    if keys.pressed(KeyCode::W) {
        cam.translation += imut_cam.forward() * speed * dt;
    }

    if keys.pressed(KeyCode::S) {
        cam.translation += imut_cam.back() * speed * dt;
    }

    if keys.pressed(KeyCode::A) {
        cam.translation += imut_cam.left() * speed * dt;
    }

    if keys.pressed(KeyCode::D) {
        cam.translation += imut_cam.right() * speed * dt;
    }

    if keys.pressed(KeyCode::LControl) {
        cam.translation -= Vec3::Y * speed * dt;
    }

    if keys.pressed(KeyCode::Space) {
        cam.translation += Vec3::Y * speed * dt;
    }
}
