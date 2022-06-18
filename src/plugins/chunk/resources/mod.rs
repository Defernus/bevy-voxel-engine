use bevy::prelude::*;

use crate::common::components::pos::{PosComponent, PosComponentAroundIterator};
use std::collections::BTreeMap;

use self::chunk::Chunk;

pub mod chunk;

pub enum InWorldChunk {
    Loaded(Chunk, Entity),
    Loading,
}

pub struct InWorldChunks(pub BTreeMap<PosComponent, Box<InWorldChunk>>);

impl InWorldChunks {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

pub struct PrevPlayerPos(pub PosComponent);
pub struct ChunkLoadIterator(pub PosComponentAroundIterator);

pub struct ChunkLoadingEnabled(pub bool);
pub struct ChunkUnloadingEnabled(pub bool);
