#![forbid(unsafe_code)]

//! World generation for Feather.
//!
//! Generation is primarily based around the `ComposableGenerator`,
//! which allows configuration of a world generator pipeline.

pub use superflat::SuperflatWorldGenerator;

use crate::base::chunk::Chunk;
use crate::base::world::{Sections, WorldHeight};
use crate::prelude::*;
use crate::BiomeList;
mod superflat;

pub trait WorldGenerator: Send + Sync {
    /// Generates the chunk at the given position.
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk;

    /// If true, has a different void fog and horizon at y=min
    fn is_flat(&self) -> bool;

    /// Creates a [`WorldGenerator`] from a config `generator_settings` string
    fn from_config(config: &str) -> Self
    where
        Self: Sized;
}

pub struct VoidWorldGenerator {
    biome: String,
}

impl WorldGenerator for VoidWorldGenerator {
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk {
        let biome = biomes
            .get_id(&self.biome)
            .unwrap_or_else(|| panic!("Biome does not exist: `{}`", self.biome));
        let mut chunk = Chunk::new(position, sections, min_y / 16);
        chunk
            .sections_mut()
            .iter_mut()
            .for_each(|s| s.biomes_mut().fill(biome));
        chunk
    }

    fn is_flat(&self) -> bool {
        true
    }

    fn from_config(config: &str) -> Self {
        VoidWorldGenerator {
            biome: config.to_string(),
        }
    }
}

/// Returns an index into a one-dimensional array
/// for the given x, y, and z values.
pub fn block_index(x: usize, y: i32, z: usize, world_height: WorldHeight, min_y: i32) -> usize {
    assert!(x < 16 && y >= min_y && y < min_y + *world_height as i32 && z < 16);
    (((y - min_y) as usize) << 8) | (x << 4) | z
}
