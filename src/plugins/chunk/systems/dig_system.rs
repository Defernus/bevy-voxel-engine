use bevy::prelude::*;
use bevy_mod_raycast::Intersection;

use crate::{
    common::components::{pos::PosComponent, ray_let::RayLet, static_mesh::StaticMeshComponent},
    plugins::chunk::{
        components::{
            chunk_state::{ChunkState, ChunkStateComponent},
            ChunkComponent,
        },
        resources::{chunk::Chunk, InWorldChunk, InWorldChunks},
    },
};

const DIG_RADIUS: f32 = 4.;
const DIG_SPEED: f32 = 0.01;
const MAX_DIG_DIST: f32 = 10.;

fn dig(
    intersection_query: &Query<&Intersection<RayLet>>,
    chunks: &mut InWorldChunks,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    dt: f32,
) -> Option<()> {
    let intersection_res = intersection_query.get_single();

    if intersection_res.is_err() {
        return None;
    }
    let intersection = intersection_res.unwrap();

    let pos = intersection.position()?;
    let dist = intersection.distance()?;
    if dist > MAX_DIG_DIST {
        return None;
    }

    let voxel_pos = PosComponent::new(pos.x as i64, pos.y as i64, pos.z as i64);
    println!("dig at {:?}", voxel_pos);

    for chunk_pos in Chunk::get_chunk_pos_by_vec(*pos).iter_neighbors(true) {
        // !TODO generate chunk if not generated yet to prevent gaps formations on chunks edges
        match chunks.0.get_mut(&chunk_pos)?.as_mut() {
            InWorldChunk::Loaded(chunk, e) => {
                let blocks_effected = chunk.dig(
                    Chunk::pos_to_relative(chunk_pos, voxel_pos),
                    DIG_RADIUS,
                    dt * DIG_SPEED,
                );
                if blocks_effected == 0 {
                    continue;
                }
                let vertices = chunk.generate_vertices(chunk_pos);
                commands.entity(*e).despawn_recursive();
                let mesh = StaticMeshComponent::spawn(commands, meshes, materials, vertices);

                *e = commands
                    .spawn()
                    .insert(ChunkComponent)
                    .insert(chunk_pos)
                    .insert(ChunkStateComponent(ChunkState::NotInitialized))
                    .add_child(mesh)
                    .id();
            }
            _ => {}
        }
    }

    Some(())
}

pub fn chunk_dig_system(
    intersection_query: Query<&Intersection<RayLet>>,
    mut chunks: ResMut<InWorldChunks>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.pressed(MouseButton::Left) {
        let dt = time.delta_seconds_f64() as f32;
        dig(
            &intersection_query,
            &mut chunks,
            &mut commands,
            &mut meshes,
            &mut materials,
            dt,
        );
    }
}
