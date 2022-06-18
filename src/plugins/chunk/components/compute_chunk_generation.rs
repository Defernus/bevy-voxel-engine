use bevy::prelude::*;
use crossbeam_channel::Receiver;

use crate::{
    common::components::{pos::PosComponent, static_mesh::vertex::Vertex},
    plugins::chunk::resources::chunk::Chunk,
};

#[derive(Component)]
pub struct ComputeChunkGeneration(pub Receiver<(PosComponent, Box<Chunk>, Vec<Vertex>)>);
