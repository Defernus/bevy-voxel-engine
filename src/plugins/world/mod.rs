use bevy::prelude::*;
pub struct WorldPlugin;

fn world_startup_system() {}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(world_startup_system);
    }
}
