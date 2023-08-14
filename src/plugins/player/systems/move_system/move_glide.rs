use bevy::prelude::*;

use crate::plugins::player::components::PlayerComponent;

const G_FORCES: f32 = 10.;

const SIDE_DRAG: f32 = 7.;
const SHIFT_SIZE_DRAG: f32 = 5.;

const DRAG: f32 = 0.5;
const SHIFT_DRAG: f32 = 0.01;

pub fn move_glide(
    keys: &Input<KeyCode>,
    player: &mut PlayerComponent,
    transform: &mut Transform,
    dt: f32,
) {
    let drag_val = if keys.pressed(KeyCode::ShiftLeft) {
        SHIFT_DRAG
    } else {
        DRAG
    };
    let side_drag_val = if keys.pressed(KeyCode::ShiftLeft) {
        SHIFT_SIZE_DRAG
    } else {
        SIDE_DRAG
    };

    let forward = transform.forward();

    let forward_speed = player.speed.dot(forward) * forward;

    let drag_a = -player.speed * drag_val;
    let g_a = -Vec3::Y * G_FORCES;
    let side_drag_a = (forward_speed - player.speed) * side_drag_val;

    let a = drag_a + g_a + side_drag_a;

    player.speed += a * dt;

    transform.translation += player.speed * dt;
}
