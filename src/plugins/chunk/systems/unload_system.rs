use bevy::prelude::*;

use crate::{
    common::components::pos::PosComponent,
    plugins::{
        chunk::resources::{chunk::Chunk, ChunkUnloadingEnabled, InWorldChunk, InWorldChunks},
        player::components::PlayerComponent,
    },
};

pub const MAX_RENDER_DISTANCE: usize = 7;

pub fn unload_chunk_system(
    mut commands: Commands,
    mut in_world_chunks: ResMut<InWorldChunks>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
    chunk_unload_enabled: Res<ChunkUnloadingEnabled>,
) {
    if !chunk_unload_enabled.0 {
        return;
    }

    let player_transform = player_transform_q.single();
    let player_pos = Chunk::get_chunk_pos_by_transform(player_transform);

    let mut chunks_to_remove: Vec<PosComponent> = vec![];
    for (chunk_pos, chunk_data) in in_world_chunks.0.iter_mut() {
        match chunk_data.as_mut() {
            InWorldChunk::Loaded(chunk, e) => {
                let delta = player_pos - *chunk_pos;
                if delta.x.abs().max(delta.y.abs()).max(delta.z.abs())
                    > MAX_RENDER_DISTANCE as i64 + 1
                {
                    chunk.clear(&mut commands);
                    commands.entity(*e).despawn_recursive();
                    chunks_to_remove.push(*chunk_pos);
                }
            }
            _ => {}
        };
    }

    for chunk_pos in chunks_to_remove {
        match in_world_chunks.0.remove(&chunk_pos) {
            Some(_) => {}
            None => panic!("failed to unload chunk: chunk position is not registered"),
        }
    }
}
