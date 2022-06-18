use bevy::prelude::*;
use bevy_mod_raycast::Intersection;

use crate::{
    common::components::{pos::PosComponent, ray_let::RayLet},
    plugins::{
        chunk::{
            components::spawn_chunk_component,
            resources::{
                chunk::{voxel::Voxel, Chunk},
                InWorldChunk, InWorldChunks,
            },
        },
        generator::resources::GeneratorRes,
    },
};

const DEFORM_RADIUS: f32 = 8.;
const DEFORM_SPEED: f32 = 0.02;
const MAX_DEFORM_DIST: f32 = 32.;

enum DeformType {
    Dig,
    Fill(Voxel),
}

fn deform_chunk(
    generator: &GeneratorRes,
    intersection_query: &Query<&Intersection<RayLet>>,
    chunks: &mut InWorldChunks,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    dt: f32,
    deform_type: DeformType,
) -> Option<()> {
    let intersection_res = intersection_query.get_single();

    if intersection_res.is_err() {
        return None;
    }
    let intersection = intersection_res.unwrap();

    let pos = intersection.position()?;
    let dist = intersection.distance()?;
    if dist > MAX_DEFORM_DIST {
        return None;
    }

    let voxel_pos = PosComponent::new(pos.x as i64, pos.y as i64, pos.z as i64);
    println!("dig at {:?}", voxel_pos);

    for chunk_pos in Chunk::get_chunk_pos_by_vec(*pos).iter_neighbors(true) {
        // !TODO generate chunk if not generated yet to prevent gaps formations on chunks edges
        match chunks.0.get_mut(&chunk_pos)?.as_mut() {
            InWorldChunk::Loaded(chunk, e) => {
                let blocks_effected = match deform_type {
                    DeformType::Dig => {
                        chunk.dig(chunk_pos, voxel_pos, DEFORM_RADIUS, dt * DEFORM_SPEED)
                    }
                    DeformType::Fill(voxel) => chunk.fill(
                        generator,
                        chunk_pos,
                        voxel_pos,
                        voxel,
                        DEFORM_RADIUS,
                        dt * DEFORM_SPEED,
                    ),
                };
                if blocks_effected == 0 {
                    continue;
                }
                let vertices = chunk.generate_vertices(chunk_pos);
                commands.entity(*e).despawn_recursive();
                *e = spawn_chunk_component(commands, meshes, materials, vertices, chunk_pos);
            }
            _ => {}
        }
    }

    Some(())
}

pub fn chunk_deform_system(
    intersection_query: Query<&Intersection<RayLet>>,
    generator: Res<GeneratorRes>,
    mut chunks: ResMut<InWorldChunks>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    buttons: Res<Input<MouseButton>>,
) {
    let dt = time.delta_seconds_f64() as f32;
    if buttons.pressed(MouseButton::Left) {
        deform_chunk(
            &generator,
            &intersection_query,
            &mut chunks,
            &mut commands,
            &mut meshes,
            &mut materials,
            dt,
            DeformType::Dig,
        );
    }
    if buttons.pressed(MouseButton::Right) {
        deform_chunk(
            &generator,
            &intersection_query,
            &mut chunks,
            &mut commands,
            &mut meshes,
            &mut materials,
            dt,
            DeformType::Fill(Voxel {
                color: Color::rgb(0.3, 0.3, 0.4),
                value: 1.,
            }),
        );
    }
}
