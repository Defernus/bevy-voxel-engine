use crate::plugins::player::components::PlayerComponent;
use bevy::prelude::*;

pub mod components;
mod systems;
pub struct PlayerPlugin;

fn player_startup_system(mut commands: Commands) {
    PlayerComponent::spawn(&mut commands);
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_startup_system)
            .add_system(systems::move_system::player_move_system)
            .add_system(systems::rotate_system::player_rotate_system);
    }
}
