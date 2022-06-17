use bevy::prelude::*;

use crate::plugins::{
    chunk::{
        components::{pos::PosComponent, ChunkComponent},
        resources::InWorldChunks,
    },
    player::components::PlayerComponent,
};

pub const MAX_RENDER_DISTANCE: usize = 7;

pub fn unload_chunk_system(
    mut commands: Commands,
    mut in_world_chunks: ResMut<InWorldChunks>,
    chunks_q: Query<(Entity, &PosComponent), With<ChunkComponent>>,
    player_transform_q: Query<&Transform, With<PlayerComponent>>,
) {
    let player_transform = player_transform_q.single();
    let player_pos = ChunkComponent::get_chunk_pos_by_transform(player_transform);

    let mut despawned = 0;

    for (e, chunk_pos) in chunks_q.iter() {
        let delta = player_pos - chunk_pos.clone();
        if delta.x.abs().max(delta.y.abs()).max(delta.z.abs()) > MAX_RENDER_DISTANCE as i64 + 1 {
            if !in_world_chunks.0.remove(&chunk_pos) {
                panic!("failed to unload chunk: chunk position is not registered");
            }

            commands.entity(e).despawn_recursive();
            despawned += 1;
        }
    }
    if despawned > 0 {
        println!("unloaded {} chunks", despawned);
    }
}
