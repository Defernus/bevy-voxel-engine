use bevy::prelude::*;

pub enum ChunkState {
    NotInitialized,
    Initializing,
    Ready,
}

#[derive(Component)]
pub struct ChunkStateComponent(pub ChunkState);
