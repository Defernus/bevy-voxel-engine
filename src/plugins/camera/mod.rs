use bevy::prelude::*;

use crate::components::camera::CameraComponent;

use self::{move_system::camera_move_system, rotate_system::camera_rotate_system};

mod move_system;
mod rotate_system;
pub struct CameraPlugin;

fn camera_startup_system(mut commands: Commands) {
    CameraComponent::spawn(&mut commands);
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_startup_system)
            .add_system(camera_move_system)
            .add_system(camera_rotate_system);
    }
}
