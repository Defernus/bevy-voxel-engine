use crate::common::components::{
    pos::PosComponent,
    static_mesh::{vertex::Vertex, StaticMeshComponent},
};
use bevy::prelude::*;

use self::chunk_state::{ChunkState, ChunkStateComponent};

pub mod chunk_state;
pub mod compute_chunk_generation;

#[derive(Component)]
pub struct ChunkComponent;

pub fn spawn_chunk_component(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    vertices: Vec<Vertex>,
    chunk_pos: PosComponent,
) -> Entity {
    let mesh = StaticMeshComponent::spawn(commands, meshes, materials, vertices);
    commands
        .spawn()
        .insert(ChunkComponent)
        .insert(chunk_pos)
        .insert(ChunkStateComponent(ChunkState::NotInitialized))
        .add_child(mesh)
        .id()
}
