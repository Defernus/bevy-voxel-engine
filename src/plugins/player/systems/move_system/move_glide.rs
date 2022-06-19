use bevy::prelude::*;

use crate::plugins::player::components::PlayerComponent;

const G_FORCES: f32 = 10.;
const DRAG: f32 = 0.5;
const SHIFT_DRAG: f32 = 0.01;

pub fn move_glide(
    keys: &Input<KeyCode>,
    player: &mut PlayerComponent,
    transform: &mut Transform,
    dt: f32,
) {
    let drag = if keys.pressed(KeyCode::LShift) {
        SHIFT_DRAG
    } else {
        DRAG
    };

    let forward = transform.forward();
    let speed = player.speed.dot(forward).max(0.)
        - forward.y * G_FORCES * dt
        - player.speed.length() * drag * dt;
    player.speed = transform.forward() * speed;

    transform.translation += player.speed * dt;
}
