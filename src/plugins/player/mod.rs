use bevy::prelude::*;

use crate::components::player::PlayerComponent;

use self::{move_system::player_move_system, rotate_system::player_rotate_system};

mod move_system;
mod rotate_system;
pub struct PlayerPlugin;

fn player_startup_system(mut commands: Commands) {
    PlayerComponent::spawn(&mut commands);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_startup_system)
            .add_system(player_move_system)
            .add_system(player_rotate_system);
    }
}
