use std::{path::PathBuf, sync::Arc};

use ahash::{AHashMap, AHashSet};
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::base::anvil::player::PlayerData;
use crate::base::chunk::Chunk;
use crate::base::chunk_lock::{ChunkHandle, ChunkLock};
use crate::base::world::{DimensionInfo, WorldHeight};
use crate::common::chunk::cache::ChunkCache;
use crate::common::chunk::worker::{ChunkWorker, LoadRequest, SaveRequest};
use crate::prelude::*;
use crate::worldgen::WorldGenerator;

#[derive(Bundle)]
pub struct WorldBundle {
    dimensions: Dimensions,
    name: WorldName,
    path: WorldPath,
}

impl WorldBundle {
    pub fn new(name: impl Into<String>, world_dir: impl Into<PathBuf>) -> WorldBundle {
        WorldBundle {
            dimensions: Dimensions(Vec::new()),
            name: WorldName(name.into()),
            path: WorldPath(world_dir.into()),
        }
    }
}

#[derive(Clone, Debug, Component, derive_more::Deref)]
pub struct WorldName(String);

#[derive(Clone, Debug, Component, derive_more::Deref)]
pub struct WorldPath(PathBuf);

impl WorldPath {
    pub fn load_player_data(&self, uuid: Uuid) -> anyhow::Result<PlayerData> {
        crate::base::anvil::player::load_player_data(&self.0, uuid)
    }

    pub fn save_player_data(&self, uuid: Uuid, data: &PlayerData) -> anyhow::Result<()> {
        crate::base::anvil::player::save_player_data(&self.0, uuid, data)
    }
}

#[derive(Component, derive_more::Deref, derive_more::DerefMut)]
pub struct Dimensions(Vec<Dimension>);

impl Dimensions {
    pub fn get(&self, dimension: &DimensionId) -> Option<&Dimension> {
        self.0.iter().find(|d| d.id() == *dimension)
    }

    pub fn get_mut(&mut self, dimension: &DimensionId) -> Option<&mut Dimension> {
        self.0.iter_mut().find(|d| d.id() == *dimension)
    }

    pub fn add(&mut self, dimension: Dimension) {
        self.0.push(dimension)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Dimension> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Dimension> {
        self.0.iter_mut()
    }
}

/// Stores all blocks and chunks in a dimension
///
/// This does not store entities; it only contains blocks.
pub struct Dimension {
    id: String,
    chunk_map: ChunkMap,
    pub cache: ChunkCache,
    chunk_worker: ChunkWorker,
    loading_chunks: AHashSet<ChunkPosition>,
    canceled_chunk_loads: AHashSet<ChunkPosition>,
    dimension_info: DimensionInfo,
    flat: bool,
}

impl Dimension {
    pub fn new(
        id: String,
        dimension_info: DimensionInfo,
        generator: Arc<dyn WorldGenerator>,
        world_dir: impl Into<PathBuf> + Clone,
        biomes: Arc<BiomeList>,
    ) -> Self {
        assert_eq!(dimension_info.info.height % 16, 0);
        Self {
            id,
            flat: generator.is_flat(),
            chunk_map: ChunkMap::new(
                WorldHeight(dimension_info.info.height as usize),
                dimension_info.info.min_y,
            ),
            cache: Default::default(),
            chunk_worker: ChunkWorker::new(
                world_dir,
                generator,
                WorldHeight(dimension_info.info.height as usize),
                dimension_info.info.min_y,
                biomes,
            ),
            loading_chunks: Default::default(),
            canceled_chunk_loads: Default::default(),
            dimension_info,
        }
    }

    /// Queues the given chunk to be loaded. If the chunk was cached, it is loaded immediately.
    pub fn queue_chunk_load(&mut self, req: LoadRequest) {
        let pos = req.pos;
        if self.cache.contains(&pos) {
            // Move the chunk from the cache to the map
            self.chunk_map
                .inner
                .insert(pos, self.cache.remove(pos).unwrap());
            self.chunk_map.chunk_handle_at(pos).unwrap().set_loaded();
        } else {
            self.loading_chunks.insert(req.pos);
            self.chunk_worker.queue_load(req);
        }
    }

    /// Loads any chunks that have been loaded asynchronously
    /// after a call to [`World::queue_chunk_load`].
    pub fn load_chunks(&mut self) -> Vec<ChunkPosition> {
        let mut events = Vec::new();
        while let Ok(Some(loaded)) = self.chunk_worker.poll_loaded_chunk() {
            self.loading_chunks.remove(&loaded.pos);
            if self.canceled_chunk_loads.remove(&loaded.pos) {
                continue;
            }
            let chunk = loaded.chunk;

            self.chunk_map.insert_chunk(chunk);
            events.push(loaded.pos);
            log::trace!("Loaded chunk {:?}", loaded.pos);
        }
        events
    }

    /// Unloads the given chunk.
    pub fn unload_chunk(&mut self, pos: ChunkPosition) -> anyhow::Result<()> {
        if let Some(handle) = self.chunk_map.inner.remove(&pos) {
            handle.set_unloaded()?;
            self.chunk_worker.queue_chunk_save(SaveRequest {
                pos,
                chunk: handle.clone(),
                entities: vec![],
                block_entities: vec![],
            });
            self.cache.insert(pos, handle);
        }
        self.chunk_map.remove_chunk(pos);
        if self.is_chunk_loading(pos) {
            self.canceled_chunk_loads.insert(pos);
        }

        Ok(())
    }

    /// Returns whether the given chunk is loaded.
    pub fn is_chunk_loaded(&self, pos: ChunkPosition) -> bool {
        self.chunk_map.inner.contains_key(&pos)
    }

    /// Returns whether the given chunk is queued to be loaded.
    pub fn is_chunk_loading(&self, pos: ChunkPosition) -> bool {
        self.loading_chunks.contains(&pos)
    }

    /// Sets the block at the given position.
    ///
    /// Returns `true` if the block was set, or `false`
    /// if its chunk was not loaded or the coordinates
    /// are out of bounds and thus no operation
    /// was performed.
    pub fn set_block_at(&self, pos: BlockPosition, block: BlockId) -> bool {
        self.chunk_map.set_block_at(pos, block)
    }

    /// Retrieves the block at the specified
    /// location. If the chunk in which the block
    /// exists is not loaded or the coordinates
    /// are out of bounds, `None` is returned.
    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        self.chunk_map.block_at(pos)
    }

    /// Returns the chunk map.
    pub fn chunk_map(&self) -> &ChunkMap {
        &self.chunk_map
    }

    /// Mutably gets the chunk map.
    pub fn chunk_map_mut(&mut self) -> &mut ChunkMap {
        &mut self.chunk_map
    }

    pub fn info(&self) -> &DimensionInfo {
        &self.dimension_info
    }

    pub fn height(&self) -> WorldHeight {
        WorldHeight(self.dimension_info.info.height as usize)
    }

    pub fn is_flat(&self) -> bool {
        self.flat
    }

    pub fn id(&self) -> DimensionId {
        DimensionId(self.id.clone())
    }
}

pub type ChunkMapInner = AHashMap<ChunkPosition, ChunkHandle>;

/// This struct stores all the chunks on the server,
/// so it allows access to blocks and lighting data.
///
/// Chunks are internally wrapped in `Arc<RwLock>`,
/// allowing multiple systems to access different parts
/// of the world in parallel. Mutable access to this
/// type is only required for inserting and removing
/// chunks.
pub struct ChunkMap {
    inner: ChunkMapInner,
    height: WorldHeight,
    min_y: i32,
}

impl ChunkMap {
    /// Creates a new, empty world.
    pub fn new(world_height: WorldHeight, min_y: i32) -> Self {
        ChunkMap {
            inner: ChunkMapInner::default(),
            height: world_height,
            min_y,
        }
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.inner.get(&pos).map(|lock| lock.read())
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at_mut(&self, pos: ChunkPosition) -> Option<RwLockWriteGuard<Chunk>> {
        self.inner.get(&pos).map(|lock| lock.write()).flatten()
    }

    /// Returns an `Arc<RwLock<Chunk>>` at the given position.
    pub fn chunk_handle_at(&self, pos: ChunkPosition) -> Option<ChunkHandle> {
        self.inner.get(&pos).map(Arc::clone)
    }

    pub fn block_at(&self, pos: BlockPosition) -> Option<BlockId> {
        if !check_coords(pos, self.height, self.min_y) {
            return None;
        }

        let (x, y, z) = chunk_relative_pos(pos);
        self.chunk_at(pos.chunk())
            .map(|chunk| chunk.block_at(x, (y - self.min_y as isize) as usize, z))
            .flatten()
    }

    pub fn set_block_at(&self, pos: BlockPosition, block: BlockId) -> bool {
        if !check_coords(pos, self.height, self.min_y) {
            return false;
        }

        let (x, y, z) = chunk_relative_pos(pos);

        self.chunk_at_mut(pos.chunk())
            .map(|mut chunk| {
                chunk.set_block_at(x, (y - self.min_y as isize) as usize, z, block, true)
            })
            .is_some()
    }

    /// Returns an iterator over chunks.
    pub fn iter_chunks(&self) -> impl IntoIterator<Item = &ChunkHandle> {
        self.inner.values()
    }

    /// Inserts a new chunk into the chunk map.
    pub fn insert_chunk(&mut self, chunk: Chunk) {
        self.inner
            .insert(chunk.position(), Arc::new(ChunkLock::new(chunk, true)));
    }

    /// Removes the chunk at the given position, returning `true` if it existed.
    pub fn remove_chunk(&mut self, pos: ChunkPosition) -> bool {
        self.inner.remove(&pos).is_some()
    }
}

fn check_coords(pos: BlockPosition, world_height: WorldHeight, min_y: i32) -> bool {
    pos.y >= min_y && pos.y < *world_height as i32 + min_y
}

fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, isize, usize) {
    (
        block_pos.x as usize & 0xf,
        block_pos.y as isize,
        block_pos.z as usize & 0xf,
    )
}
