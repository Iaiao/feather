use crate::prelude::*;
use ahash::AHashMap;

/// Data structure to query which clients should
/// receive updates from a given chunk, fast.
#[derive(Default)]
pub struct ChunkSubscriptions {
    pub chunks: AHashMap<DimensionChunkPosition, Vec<ClientId>>,
}

impl ChunkSubscriptions {
    pub fn subscriptions_for(&self, chunk: &DimensionChunkPosition) -> &[ClientId] {
        self.chunks
            .get(chunk)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}
