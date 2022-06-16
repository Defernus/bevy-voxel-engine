use crate::{
    components::{
        chunk::{
            pos::{PosComponent, PosComponentAroundIterator},
            ChunkComponent,
        },
        player::PlayerComponent,
    },
    resources::generator::GeneratorResource,
};
use bevy::prelude::*;
use std::collections::BTreeSet;

pub const DEFAULT_RADIUS: usize = 5;

pub struct PrevPlayerPos(pub PosComponent);
pub struct ChunkLoadIterator(pub PosComponentAroundIterator);

impl ChunkLoadIterator {
    pub fn new(pos: PosComponent) -> Self {
        Self(pos.iter_around(DEFAULT_RADIUS))
    }
}

fn generate_chunk(
    chunk_load_iter: &mut ChunkLoadIterator,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    generator: &GeneratorResource,
    existed_chunks: &BTreeSet<PosComponent>,
) -> Option<()> {
    let mut chunk_to_load = chunk_load_iter.0.next()?;

    while existed_chunks.contains(&chunk_to_load) {
        chunk_to_load = chunk_load_iter.0.next()?
    }

    ChunkComponent::spawn(commands, meshes, materials, generator, chunk_to_load);

    Some(())
}

pub fn chunk_load_system(
    mut prev_player_chunk_pos: ResMut<PrevPlayerPos>,
    mut chunk_load_iter: ResMut<ChunkLoadIterator>,
    chunks_q: Query<&PosComponent, With<ChunkComponent>>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    generator: Res<GeneratorResource>,
) {
    let player_transform = player_transform_q.single();
    let mut existed_chunks: BTreeSet<PosComponent> = BTreeSet::new();
    for pos in chunks_q.iter() {
        existed_chunks.insert(pos.clone());
    }

    let player_chunk_pos = ChunkComponent::get_chunk_pos_by_transform(player_transform);

    if player_chunk_pos != prev_player_chunk_pos.0 {
        println!("player chunk position changed");
        prev_player_chunk_pos.0 = player_chunk_pos;
        chunk_load_iter.0 = player_chunk_pos.iter_around(DEFAULT_RADIUS);
    }

    generate_chunk(
        &mut chunk_load_iter,
        &mut commands,
        &mut meshes,
        &mut materials,
        &generator,
        &existed_chunks,
    );
}
