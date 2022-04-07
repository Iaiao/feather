use crate::prelude::*;
use ahash::AHashMap;

/// Stores the players waiting on chunks that are currently being loaded.
#[derive(Default)]
pub struct WaitingChunks(AHashMap<DimensionChunkPosition, Vec<Entity>>);

impl WaitingChunks {
    pub fn drain_players_waiting_for(&mut self, chunk: DimensionChunkPosition) -> Vec<Entity> {
        self.0.remove(&chunk).unwrap_or_default()
    }

    pub fn insert(&mut self, player: Entity, chunk: DimensionChunkPosition) {
        self.0.entry(chunk).or_default().push(player);
    }

    pub fn remove_player(&mut self, player: Entity) {
        for (_, players) in self.0.iter_mut() {
            if let Some(index) = players.iter().position(|&p| p == player) {
                players.remove(index);
            }
        }
        self.0.retain(|_, players| !players.is_empty());
    }
}
