use bevy::prelude::*;

use self::systems::{
    move_system::player_move_system, player_startup_system, rotate_system::player_rotate_system,
};

pub mod components;
mod systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_startup_system)
            .add_system(player_move_system)
            .add_system(player_rotate_system);
    }
}
