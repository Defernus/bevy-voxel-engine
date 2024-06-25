use bevy::prelude::*;

use crate::plugins::chunk::resources::{ChunkLoadingEnabled, ChunkUnloadingEnabled};

pub fn chunk_loading_control_system(
    mut chunk_load_enabled: ResMut<ChunkLoadingEnabled>,
    mut chunk_unload_enabled: ResMut<ChunkUnloadingEnabled>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::BracketRight) {
        chunk_load_enabled.0 = !chunk_load_enabled.0;
        println!("chunk_load_enabled: {}", chunk_load_enabled.0);
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        chunk_unload_enabled.0 = !chunk_unload_enabled.0;
        println!("chunk_unload_enabled: {}", chunk_unload_enabled.0);
    }
}
