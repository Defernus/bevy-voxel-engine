use crate::common::components::ray_let::RayLet;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::Projection};
use bevy_mod_raycast::prelude::RaycastSource;

use super::components::CameraComponent;

pub fn camera_startup_system(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb_u8(0x47, 0x48, 0x50)),
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
