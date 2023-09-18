use crate::common::components::{
    pos::PosComponent,
    ray_let::RayLet,
    static_mesh::{vertex::Vertex, StaticMeshComponent},
};
use bevy::prelude::*;
use bevy_mod_raycast::RaycastMesh;

use self::chunk_state::{ChunkState, ChunkStateComponent};

pub mod chunk_object;
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
        .entity(mesh)
        .insert(RaycastMesh::<RayLet>::default());
    commands
        .spawn(ChunkComponent)
        .insert(chunk_pos)
        .insert(ChunkStateComponent(ChunkState::NotInitialized))
        .insert(SpatialBundle::default())
        .add_child(mesh)
        .id()
}
