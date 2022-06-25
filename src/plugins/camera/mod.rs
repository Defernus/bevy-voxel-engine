use bevy::{prelude::*, sprite::Material2dPlugin};

use self::{
    components::post_processing_material::PostProcessingMaterial, systems::camera_startup_system,
};

pub mod components;
mod systems;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_startup_system(camera_startup_system);
    }
}
