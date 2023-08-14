use bevy::prelude::*;

use crate::common::components::pos::{PosComponent, PosComponentAroundIterator};
use std::collections::BTreeMap;

use self::chunk::Chunk;

pub mod chunk;

pub enum InWorldChunk {
    Loaded(Box<Chunk>, Entity),
    Loading,
}

#[derive(Resource)]
pub struct InWorldChunks(pub BTreeMap<PosComponent, Box<InWorldChunk>>);

impl InWorldChunks {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

#[derive(Resource)]
pub struct PrevPlayerPos(pub PosComponent);
#[derive(Resource)]
pub struct ChunkLoadIterator(pub PosComponentAroundIterator);

#[derive(Resource)]
pub struct ChunkLoadingEnabled(pub bool);
#[derive(Resource)]
pub struct ChunkUnloadingEnabled(pub bool);
