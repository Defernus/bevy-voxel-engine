use bevy::prelude::*;

use crate::components::camera::CameraComponent;

pub struct CameraPlugin;

fn camera_startup_system(mut commands: Commands) {
    CameraComponent::spawn(&mut commands);
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_startup_system);
    }
}
