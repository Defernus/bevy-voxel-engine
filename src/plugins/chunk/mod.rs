use bevy::prelude::*;

use crate::common::components::pos::PosComponent;

use self::resources::{
    ChunkLoadIterator, ChunkLoadingEnabled, ChunkUnloadingEnabled, InWorldChunks, PrevPlayerPos,
};
use self::systems::load_system::{chunk_load_system, spawn_chunk_system};
use self::systems::loading_control_system::chunk_loading_control_system;
use self::systems::unload_system::unload_chunk_system;

pub mod components;
pub mod resources;
mod systems;
pub struct ChunkPlugin;

fn chunk_startup_system() {}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InWorldChunks::new())
            .add_startup_system(chunk_startup_system)
            .insert_resource(PrevPlayerPos(PosComponent::new(0, 0, 0)))
            .insert_resource(ChunkLoadingEnabled(true))
            .insert_resource(ChunkUnloadingEnabled(true))
            .insert_resource(ChunkLoadIterator::new(PosComponent::new(0, 0, 0)))
            .add_system(chunk_loading_control_system)
            .add_system(chunk_load_system)
            .add_system(unload_chunk_system)
            .add_system(spawn_chunk_system);
    }
}
