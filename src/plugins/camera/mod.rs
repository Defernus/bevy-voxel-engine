use bevy::prelude::*;

use self::systems::{camera_startup_system, follow_system::camera_follow_system};

pub mod components;
mod systems;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_startup_system)
            .add_system(camera_follow_system);
    }
}
