use std::collections::BTreeSet;

use super::components::pos::{PosComponent, PosComponentAroundIterator};

pub struct InWorldChunks(pub BTreeSet<PosComponent>);

impl InWorldChunks {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
}

pub struct PrevPlayerPos(pub PosComponent);
pub struct ChunkLoadIterator(pub PosComponentAroundIterator);
