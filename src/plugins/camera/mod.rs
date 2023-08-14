use bevy::prelude::*;

use self::systems::camera_startup_system;

pub mod components;
mod systems;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_startup_system);
    }
}
