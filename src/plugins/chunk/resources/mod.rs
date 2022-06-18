use std::collections::BTreeSet;

use crate::common::components::pos::{PosComponent, PosComponentAroundIterator};

pub struct InWorldChunks(pub BTreeSet<PosComponent>);

impl InWorldChunks {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
}

pub struct PrevPlayerPos(pub PosComponent);
pub struct ChunkLoadIterator(pub PosComponentAroundIterator);

pub struct ChunkLoadingEnabled(pub bool);
pub struct ChunkUnloadingEnabled(pub bool);
