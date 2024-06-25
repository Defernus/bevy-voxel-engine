use bevy::prelude::*;

use crate::plugins::player::components::PlayerComponent;

pub fn player_control_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut PlayerComponent>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let mut player = player_q.get_single_mut().expect("player not spawned yet");
        player.noclip = !player.noclip;
    }
}
