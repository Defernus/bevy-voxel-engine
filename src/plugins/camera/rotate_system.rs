use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::components::camera::CameraComponent;

pub(super) fn camera_rotate_system(
    mut ev_motion: EventReader<MouseMotion>,
    windows: Res<Windows>,
    mut camera: Query<&mut Transform, With<CameraComponent>>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let mut transform = camera.get_single_mut().expect("camera does not exists yet");

    let mut rotation_move = Vec2::ZERO;

    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }

    let delta_x = rotation_move.x / window_size.x * std::f32::consts::PI * 2.0;
    let delta_y = rotation_move.y / window_size.y * std::f32::consts::PI;
    let yaw = Quat::from_rotation_y(-delta_x);
    let pitch = Quat::from_rotation_x(-delta_y);
    transform.rotation = yaw * transform.rotation; // rotate around global y axis
    transform.rotation = transform.rotation * pitch; // rotate around local x axis
}
