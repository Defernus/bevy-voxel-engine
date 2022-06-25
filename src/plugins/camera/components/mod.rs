use bevy::prelude::*;

#[derive(Component)]
pub struct CameraComponent;

pub fn update_camera_transform(player: Transform, camera: &mut Transform) {
    *camera = player;
}
