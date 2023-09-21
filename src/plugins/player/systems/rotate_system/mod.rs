use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::plugins::{
    camera::components::{update_camera_transform, CameraComponent},
    player::components::PlayerComponent,
};

pub fn player_rotate_system(
    mut ev_motion: EventReader<MouseMotion>,
    windows: Query<&Window>,
    mut camera: Query<&mut Transform, (With<CameraComponent>, Without<PlayerComponent>)>,
    mut player: Query<(&mut Transform, &PlayerComponent), Without<CameraComponent>>,
) {
    let mut camera = camera.get_single_mut().expect("camera does not exists yet");
    let camera = camera.as_mut();

    let window = windows.get_single().unwrap();
    let window_size = Vec2::new(window.width(), window.height());
    let (mut transform, _) = player.get_single_mut().expect("player does not exists yet");

    let mut rotation_move = Vec2::ZERO;

    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }

    let delta_x = rotation_move.x / window_size.x * std::f32::consts::PI;
    let delta_y = rotation_move.y / window_size.x * std::f32::consts::PI;
    let yaw = Quat::from_rotation_y(-delta_x);
    let pitch = Quat::from_rotation_x(-delta_y);
    transform.rotation *= yaw;
    transform.rotation *= pitch;

    update_camera_transform(*transform, camera);
}
