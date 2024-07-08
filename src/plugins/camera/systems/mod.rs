use crate::common::components::ray_let::RayLet;
use bevy::{
    prelude::*,
    render::camera::{ClearColorConfig, Projection},
};
use bevy_mod_raycast::prelude::RaycastSource;

use super::components::CameraComponent;

pub fn camera_startup_system(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::srgb_u8(0x47, 0x48, 0x50)),
                ..default()
            },
            transform: Transform::default(),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 2.0,
                ..default()
            }),
            ..default()
        })
        .insert(CameraComponent)
        .insert(RaycastSource::<RayLet>::new_transform_empty());
}
