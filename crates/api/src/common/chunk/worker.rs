use std::{path::PathBuf, sync::Arc};

use anyhow::bail;
use flume::{Receiver, Sender};

use crate::base::anvil::{block_entity::BlockEntityData, entity::EntityData};
use crate::base::chunk::Chunk;
use crate::base::chunk_lock::ChunkHandle;
use crate::base::world::{Sections, WorldHeight};
use crate::common::positions::ChunkPosition;
use crate::common::region_worker::RegionWorker;
use crate::worldgen::WorldGenerator;
use crate::BiomeList;

#[derive(Debug)]
pub struct LoadRequest {
    pub pos: ChunkPosition,
}
#[derive(Debug)]
pub struct LoadedChunk {
    pub pos: ChunkPosition,
    pub chunk: Chunk,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ChunkLoadResult {
    /// The chunk does not exist in this source.
    Missing(ChunkPosition),
    /// An error occurred while loading the chunk.
    Error(anyhow::Error),
    /// Successfully loaded the chunk.
    Loaded(LoadedChunk),
}

#[derive(Debug)]
pub struct SaveRequest {
    pub pos: ChunkPosition,
    pub chunk: ChunkHandle,
    pub entities: Vec<EntityData>,
    pub block_entities: Vec<BlockEntityData>,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum WorkerRequest {
    Load(LoadRequest),
    Save(SaveRequest),
}
pub struct ChunkWorker {
    generator: Arc<dyn WorldGenerator>,
    send_req: Sender<WorkerRequest>,
    send_gen: Sender<LoadedChunk>,
    recv_gen: Receiver<LoadedChunk>, // Chunk generation should be infallible.
    recv_load: Receiver<ChunkLoadResult>,
    sections: Sections,
    min_y: i32,
    biomes: Arc<BiomeList>,
}

impl ChunkWorker {
    pub fn new(
        world_dir: impl Into<PathBuf>,
        generator: Arc<dyn WorldGenerator>,
        height: WorldHeight,
        min_y: i32,
        biomes: Arc<BiomeList>,
    ) -> Self {
        let (send_req, recv_req) = flume::unbounded();
        let (send_gen, recv_gen) = flume::unbounded();
        let (region_worker, recv_load) =
            RegionWorker::new(world_dir.into(), recv_req, height, biomes.clone());
        region_worker.start();
        Self {
            generator,
            send_req,
            send_gen,
            recv_gen,
            recv_load,
            sections: height.into(),
            min_y,
            biomes,
        }
    }
    pub fn queue_load(&mut self, request: LoadRequest) {
        self.send_req.send(WorkerRequest::Load(request)).unwrap()
    }

    /// Helper function for poll_loaded_chunk. Attempts to receive a freshly generated chunk.
    /// Function signature identical to that of poll_loaded_chunk for ease of use.
    fn try_recv_gen(&mut self) -> Result<Option<LoadedChunk>, anyhow::Error> {
        match self.recv_gen.try_recv() {
            Ok(l) => Ok(Some(l)),
            Err(e) => match e {
                flume::TryRecvError::Empty => Ok(None),
                flume::TryRecvError::Disconnected => bail!("chunkgen channel died"),
            },
        }
    }
    pub fn poll_loaded_chunk(&mut self) -> Result<Option<LoadedChunk>, anyhow::Error> {
        match self.recv_load.try_recv() {
            Ok(answer) => {
                match answer {
                    // RegionWorker answered
                    ChunkLoadResult::Missing(pos) => {
                        // chunk does not exist, queue it for generation
                        let send_gen = self.send_gen.clone();
                        let gen = self.generator.clone();
                        let sections = self.sections;
                        let min_y = self.min_y;
                        let biomes = self.biomes.clone();
                        rayon::spawn(move || {
                            // spawn task to generate chunk
                            let chunk = gen.generate_chunk(pos, sections, min_y, &*biomes);
                            send_gen.send(LoadedChunk { pos, chunk }).unwrap()
                        });
                        self.try_recv_gen() // check for generated chunks
                    }
                    ChunkLoadResult::Error(e) => Err(e),
                    ChunkLoadResult::Loaded(l) => Ok(Some(l)),
                }
            }
            Err(e) => match e {
                flume::TryRecvError::Empty => self.try_recv_gen(), // check for generated chunks
                flume::TryRecvError::Disconnected => bail!("RegionWorker died"),
            },
        }
    }

    pub fn queue_chunk_save(&mut self, req: SaveRequest) {
        self.send_req.send(WorkerRequest::Save(req)).unwrap()
    }
}