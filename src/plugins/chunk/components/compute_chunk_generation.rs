use bevy::prelude::*;
use crossbeam_channel::Receiver;

use crate::common::components::{pos::PosComponent, static_mesh::vertex::Vertex};

use super::ChunkComponent;

#[derive(Component)]
pub struct ComputeChunkGeneration(pub Receiver<(PosComponent, Box<ChunkComponent>, Vec<Vertex>)>);
