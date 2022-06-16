use bevy::prelude::*;

use crate::components::chunk::pos::PosComponent;

use self::load_system::{chunk_load_system, ChunkLoadIterator, PrevPlayerPos};

mod load_system;
pub struct ChunkPlugin;

fn chunk_startup_system() {}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(chunk_startup_system)
            .insert_resource(PrevPlayerPos(PosComponent::new(0, 0, 0)))
            .insert_resource(ChunkLoadIterator::new(PosComponent::new(0, 0, 0)))
            .add_system(chunk_load_system);
    }
}
