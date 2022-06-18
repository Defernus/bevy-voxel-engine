use bevy::prelude::*;

#[derive(Component)]
pub struct CameraComponent;

pub fn update_camera_transform(player: Transform, camera: &mut Transform) {
    camera.scale = player.scale;
    camera.rotation = player.rotation;
    camera.translation = player.translation + player.back() * 10.;
}
