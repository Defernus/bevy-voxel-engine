use bevy::prelude::*;

use crate::plugins::player::components::PlayerComponent;

const NOCLIP_BASE_SPEED: f32 = 10.;
const NOCLIP_SHIFT_SPEED: f32 = 50.;

pub fn move_noclip(
    keys: &ButtonInput<KeyCode>,
    player: &mut PlayerComponent,
    transform: &mut Transform,
    dt: f32,
) {
    let imut_transform = transform.clone();
    player.speed = Vec3::ZERO;

    let speed = if keys.pressed(KeyCode::ShiftLeft) {
        NOCLIP_SHIFT_SPEED
    } else {
        NOCLIP_BASE_SPEED
    };

    if keys.pressed(KeyCode::KeyW) {
        transform.translation += imut_transform.forward() * speed * dt;
    }

    if keys.pressed(KeyCode::KeyS) {
        transform.translation += imut_transform.back() * speed * dt;
    }

    if keys.pressed(KeyCode::KeyA) {
        transform.translation += imut_transform.left() * speed * dt;
    }

    if keys.pressed(KeyCode::KeyD) {
        transform.translation += imut_transform.right() * speed * dt;
    }

    if keys.pressed(KeyCode::ControlLeft) {
        transform.translation -= Vec3::Y * speed * dt;
    }

    if keys.pressed(KeyCode::Space) {
        transform.translation += Vec3::Y * speed * dt;
    }
}
