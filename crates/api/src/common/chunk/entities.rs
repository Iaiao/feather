use crate::prelude::*;
use ahash::AHashMap;

/// A spatial index to look up entities within a given chunk.
#[derive(Default)]
pub struct ChunkEntities {
    entities: AHashMap<ChunkPosition, Vec<Entity>>,
}

impl ChunkEntities {
    /// Returns the entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &[Entity] {
        self.entities
            .get(&chunk)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub fn update(
        &mut self,
        entity: Entity,
        old_chunk: Option<ChunkPosition>,
        new_chunk: ChunkPosition,
    ) {
        if let Some(old_chunk) = old_chunk {
            self.remove_entity(entity, old_chunk);
        }

        self.entities.entry(new_chunk).or_default().push(entity);
    }

    pub fn remove_entity(&mut self, entity: Entity, chunk: ChunkPosition) {
        if let Some(vec) = self.entities.get_mut(&chunk) {
            if let Some(i) = vec.iter().position(|&e| e == entity) {
                vec.remove(i);
            }
        }
    }
}
