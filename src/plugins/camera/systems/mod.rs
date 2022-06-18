use crate::common::components::ray_let::RayLet;
use bevy::{prelude::*, render::camera::Projection};
use bevy_mod_raycast::RayCastSource;

use super::components::CameraComponent;

pub mod follow_system;

pub fn camera_startup_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::default(),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: std::f32::consts::PI / 2.0,
                ..default()
            }),
            ..default()
        })
        .insert(CameraComponent)
        .insert(RayCastSource::<RayLet>::new_transform_empty());
}
