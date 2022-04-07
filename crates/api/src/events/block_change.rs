use std::iter;

use ahash::AHashMap;

use crate::base::chunk::SECTION_HEIGHT;
use crate::{BlockPosition, ChunkPosition, DimensionId, WorldId};

/// Event triggered when one or more blocks are changed.
///
/// This event can efficiently store bulk block updates
/// using a variety of different representations.
#[derive(Debug, Clone)]
pub struct BlockChangeEvent {
    pub changes: BlockChanges,
    world: WorldId,
    dimension: DimensionId,
}

impl BlockChangeEvent {
    /// Creates an event affecting a single block.
    pub fn single(pos: BlockPosition, world: WorldId, dimension: DimensionId) -> Self {
        Self {
            changes: BlockChanges::Single { pos },
            world,
            dimension,
        }
    }

    /// Creates an event affecting multiple blocks.
    pub fn multiple(positions: Vec<BlockPosition>, world: WorldId, dimension: DimensionId) -> Self {
        Self {
            changes: BlockChanges::Multiple { positions },
            world,
            dimension,
        }
    }

    /// Determines the number of blocks that were
    /// changed in this block change event.
    pub fn count(&self) -> usize {
        match &self.changes {
            BlockChanges::Single { .. } => 1,
            BlockChanges::Multiple { positions } => positions.len(),
        }
    }

    /// Returns an iterator over block positions affected by this block change.
    pub fn iter_changed_blocks(&self) -> Box<dyn Iterator<Item = BlockPosition> + '_> {
        match &self.changes {
            BlockChanges::Single { pos } => Box::new(iter::once(*pos)),
            BlockChanges::Multiple { positions } => Box::new(positions.iter().copied()),
        }
    }

    /// Returns an iterator over chunk section positions affected by this block change.
    ///
    /// The yielded tuple consists of `(chunk, section_y, num_changed_blocks)`,
    /// where `num_changed_blocks` is the number of blocks changed within that chunk.
    pub fn iter_affected_chunk_sections(
        &self,
    ) -> Box<dyn Iterator<Item = (ChunkPosition, i32, usize)> + '_> {
        match &self.changes {
            BlockChanges::Single { pos } => {
                Box::new(iter::once((pos.chunk(), pos.y / SECTION_HEIGHT as i32, 1)))
            }
            BlockChanges::Multiple { positions } => Box::new({
                let mut sections = AHashMap::new();
                for pos in positions {
                    *sections.entry((pos.chunk(), pos.y / 16)).or_default() += 1;
                }
                sections
                    .into_iter()
                    .map(|((chunk, y), changed)| (chunk, y, changed))
            }),
        }
    }

    pub fn dimension(&self) -> &DimensionId {
        &self.dimension
    }

    pub fn world(&self) -> WorldId {
        self.world
    }
}

#[derive(Debug, Clone)]
pub enum BlockChanges {
    /// A single block change.
    Single {
        pos: BlockPosition,
    },
    Multiple {
        positions: Vec<BlockPosition>,
    },
}

#[cfg(test)]
mod tests {
    use std::collections::hash_set::HashSet;

    use crate::Entity;

    use super::*;

    #[test]
    fn single() {
        let pos = BlockPosition::new(5, 64, 9).try_into().unwrap();
        let event = BlockChangeEvent::single(
            pos,
            WorldId(Entity::from_raw(0)), // doesn't matter
            DimensionId("minecraft:overworld".to_string()),
        );
        assert_eq!(event.count(), 1);
        assert_eq!(event.iter_changed_blocks().collect::<Vec<_>>(), vec![pos]);
        assert_eq!(
            event.iter_affected_chunk_sections().collect::<Vec<_>>(),
            vec![(pos.chunk(), 4, 1)]
        );
    }

    #[test]
    fn multiple() {
        let event = BlockChangeEvent::multiple(
            vec![
                BlockPosition::new(0, 0, 0),
                BlockPosition::new(1, 6, 0),
                BlockPosition::new(2, 0, 6),
                BlockPosition::new(18, 99, 0),
                BlockPosition::new(-1, -6, 306),
            ],
            WorldId(Entity::from_raw(0)), // doesn't matter
            DimensionId("minecraft:overworld".to_string()),
        );
        assert_eq!(event.count(), 5);
        assert_eq!(event.iter_changed_blocks().count(), 5);
        assert_eq!(
            event.iter_affected_chunk_sections().collect::<HashSet<_>>(),
            HashSet::from_iter([
                (ChunkPosition { x: -1, z: 19 }, 0, 1),
                (ChunkPosition { x: 1, z: 0 }, 6, 1),
                (ChunkPosition { x: 0, z: 0 }, 0, 3)
            ])
        );
    }
}
