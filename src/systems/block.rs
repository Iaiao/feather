//! Implements block change broadcasting.
//!
//! # Bulk updates
//! The protocol provides three methods to change blocks
//! on the client:
//! * The `BlockChange` packet to update a single block.
//! * The `MultiBlockChange` packet to update multiple blocks
//! within a single chunk section.
//! * The `ChunkData` packet to overwrite entire chunk sections
//! at once.
//!
//! Feather is optimized for bulk block updates to cater to plugins
//! like WorldEdit. This module chooses the optimal packet from
//! the above three options to achieve ideal performance.

use ahash::AHashMap;

use feather_api::base::chunk::{SECTION_HEIGHT, SECTION_VOLUME};
use feather_api::common::world::Dimensions;
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(broadcast_block_changes);
}

fn broadcast_block_changes(
    server: Res<Server>,
    worlds: Query<&Dimensions>,
    mut event_reader: EventReader<BlockChangeEvent>,
) {
    for event in event_reader.iter() {
        broadcast_block_change(
            event,
            &*server,
            worlds
                .get(*event.world())
                .expect("Invalid world passed to BlockChangeEvent")
                .get(event.dimension())
                .expect("Invalid dimension passed to BlockChangeEvent"),
        );
    }
}

/// Threshold at which to switch from block change to chunk
// overwrite packets.
const CHUNK_OVERWRITE_THRESHOLD: usize = SECTION_VOLUME / 2;

fn broadcast_block_change(event: &BlockChangeEvent, server: &Server, dimension: &Dimension) {
    match event.changes {
        BlockChanges::Single { pos } => {
            if let Some(new_block) = dimension.block_at(pos) {
                server.broadcast_nearby_with(
                    event.world(),
                    (*event.dimension()).clone(),
                    pos.position(),
                    |client| client.send_block_change(pos.try_into().unwrap(), new_block),
                );
            }
        }
        BlockChanges::Multiple { .. } => {
            let mut map = AHashMap::<ChunkPosition, Vec<BlockPosition>>::new();
            for block in event.iter_changed_blocks() {
                map.entry(block.chunk()).or_default().push(block);
            }
            for (chunk, blocks) in map {
                if let Some(chunk_handle) = dimension.chunk_map().chunk_handle_at(chunk) {
                    if blocks.len() >= CHUNK_OVERWRITE_THRESHOLD {
                        let position = position!(
                            (chunk.x * CHUNK_WIDTH as i32) as f64,
                            0.0,
                            (chunk.z * CHUNK_WIDTH as i32) as f64,
                        );
                        server.broadcast_nearby_with(
                            WorldId(*event.world()),
                            DimensionId(event.dimension().to_string()),
                            position,
                            |client| {
                                client.overwrite_chunk(&chunk_handle);
                            },
                        )
                    } else {
                        let mut map = AHashMap::<usize, Vec<(u8, u8, u8)>>::new();
                        for block in blocks {
                            map.entry(
                                (block.y - dimension.info().info.min_y) as usize / SECTION_HEIGHT,
                            )
                            .or_default()
                            .push((
                                (block.x & 0xf) as u8,
                                (block.y & 0xf) as u8,
                                (block.z & 0xf) as u8,
                            ));
                        }
                        for (y, blocks) in map {
                            server.broadcast_nearby_with(
                                WorldId(*event.world()),
                                DimensionId(event.dimension().to_string()),
                                chunk.first_block().position(),
                                |client| {
                                    client.send_multi_block_change(&chunk_handle, &blocks, y);
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
