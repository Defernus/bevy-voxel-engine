use crate::{
    common::components::pos::PosComponent,
    plugins::{
        chunk::{
            components::{
                chunk_state::{ChunkState, ChunkStateComponent},
                compute_chunk_generation::ComputeChunkGeneration,
                static_mesh::StaticMeshComponent,
                ChunkComponent,
            },
            resources::{ChunkLoadIterator, ChunkLoadingEnabled, InWorldChunks, PrevPlayerPos},
        },
        generator::resources::GeneratorRes,
        player::components::PlayerComponent,
    },
};
use bevy::prelude::*;
use crossbeam_channel::unbounded;

pub const DEFAULT_RADIUS: usize = 5;
pub const CHUNKS_SPAWN_AT_ONCE: usize = 4;

impl ChunkLoadIterator {
    pub fn new(pos: PosComponent) -> Self {
        Self(pos.iter_around(DEFAULT_RADIUS))
    }
}

fn generate_chunk(
    in_world_chunks: &mut InWorldChunks,
    chunk_load_iter: &mut ChunkLoadIterator,
    commands: &mut Commands,
    generator: &GeneratorRes,
) -> Option<()> {
    for _ in 0..CHUNKS_SPAWN_AT_ONCE {
        let mut pos = chunk_load_iter.0.next()?;

        while in_world_chunks.0.contains(&pos) {
            pos = chunk_load_iter.0.next()?
        }
        in_world_chunks.0.insert(pos);

        let (tx, rx) = unbounded();

        let gen = generator.clone();
        std::thread::spawn(move || {
            let mut chunk = ChunkComponent::new(&gen, pos);
            let vertices = chunk.generate_vertices(pos);

            match tx.send((pos, Box::new(chunk), vertices)) {
                Err(err) => {
                    panic!("failed to send chunk data after generation: {}", err);
                }
                _ => {}
            }
        });

        commands.spawn().insert(ComputeChunkGeneration(rx));
    }
    Some(())
}

pub fn chunk_load_system(
    mut in_world_chunks: ResMut<InWorldChunks>,
    mut prev_player_chunk_pos: ResMut<PrevPlayerPos>,
    mut chunk_load_iter: ResMut<ChunkLoadIterator>,
    chunk_load_enabled: Res<ChunkLoadingEnabled>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,

    mut commands: Commands,
    generator: Res<GeneratorRes>,
) {
    if !chunk_load_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();

    let player_chunk_pos = ChunkComponent::get_chunk_pos_by_transform(player_transform);

    if player_chunk_pos != prev_player_chunk_pos.0 {
        prev_player_chunk_pos.0 = player_chunk_pos;
        chunk_load_iter.0 = player_chunk_pos.iter_around(DEFAULT_RADIUS);
    }

    if chunk_load_iter.0.is_done() {
        return;
    }

    generate_chunk(
        &mut in_world_chunks,
        &mut chunk_load_iter,
        &mut commands,
        &generator,
    );
}

pub fn spawn_chunk_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    generation_task: Query<(Entity, &mut ComputeChunkGeneration)>,
) {
    for (e, ComputeChunkGeneration(rx)) in generation_task.iter() {
        match rx.try_recv() {
            Ok((pos, chunk, vertices)) => {
                let mesh = StaticMeshComponent::spawn(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    vertices,
                );

                commands
                    .spawn()
                    .insert(*chunk)
                    .insert(pos)
                    .insert(ChunkStateComponent(ChunkState::NotInitialized))
                    .add_child(mesh);
                commands.entity(e).despawn();
            }
            _ => {}
        }
    }
}
