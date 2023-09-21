use bevy::prelude::*;

use crate::common::components::pos::PosComponent;

use self::resources::chunk::object::handlers::ObjectHandlers;
use self::resources::{
    ChunkLoadIterator, ChunkLoadingEnabled, ChunkUnloadingEnabled, InWorldChunks, PrevPlayerPos,
};
use self::systems::deform_system::chunk_deform_system;
use self::systems::load_system::{chunk_load_system, spawn_chunk_system};
use self::systems::loading_control_system::chunk_loading_control_system;
use self::systems::unload_system::unload_chunk_system;

pub mod components;
pub mod resources;
mod systems;
pub struct ChunkPlugin;

fn chunk_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ObjectHandlers::new(&mut meshes, &mut materials));
}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InWorldChunks::new())
            .add_systems(Startup, chunk_startup_system)
            .insert_resource(PrevPlayerPos(PosComponent::new(0, 0, 0)))
            .insert_resource(ChunkLoadingEnabled(true))
            .insert_resource(ChunkUnloadingEnabled(true))
            .insert_resource(ChunkLoadIterator::new(PosComponent::new(0, 0, 0)))
            .add_systems(Update, chunk_loading_control_system)
            .add_systems(Update, chunk_load_system)
            .add_systems(Update, unload_chunk_system)
            .add_systems(Update, chunk_deform_system)
            .add_systems(Update, spawn_chunk_system);
    }
}
