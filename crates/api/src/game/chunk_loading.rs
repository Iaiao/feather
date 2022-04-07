use crate::prelude::*;
use ahash::AHashMap;
use std::collections::VecDeque;
use std::mem;
use std::time::{Duration, Instant};

/// Amount of time to wait after a chunk has
/// no tickets until it is unloaded.
const UNLOAD_DELAY: Duration = Duration::from_secs(10); // TODO make it configurable

#[derive(Default)]
pub struct ChunkLoadState {
    /// Chunks that have been queued for unloading.
    pub chunk_unload_queue: VecDeque<QueuedChunkUnload>,

    pub chunk_tickets: ChunkTickets,
}

impl ChunkLoadState {
    pub fn remove_ticket(&mut self, chunk: &DimensionChunkPosition, ticket: Ticket) {
        self.chunk_tickets.remove_ticket(chunk, ticket);

        // If this was the last ticket, then queue the chunk to be
        // unloaded.
        if self.chunk_tickets.num_tickets(chunk) == 0 {
            self.chunk_tickets.remove_chunk(chunk);
            self.chunk_unload_queue
                .push_back(QueuedChunkUnload::new(chunk.clone()));
        }
    }

    pub fn remove_all_tickets(&mut self, ticket: Ticket) {
        for pos in self.chunk_tickets.take_tickets(ticket) {
            self.remove_ticket(&pos, ticket);
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueuedChunkUnload {
    pub pos: DimensionChunkPosition,
    /// Time after which the chunk should be unloaded.
    pub unload_at_time: Instant,
}

impl QueuedChunkUnload {
    pub fn new(pos: DimensionChunkPosition) -> Self {
        Self {
            pos,
            unload_at_time: Instant::now() + UNLOAD_DELAY,
        }
    }
}

/// Maintains a list of "tickets" for each loaded chunk.
/// A chunk is queued for unloading when it has no more tickets.
#[derive(Default)]
pub struct ChunkTickets {
    tickets: AHashMap<DimensionChunkPosition, Vec<Ticket>>,
    by_entity: AHashMap<Ticket, Vec<DimensionChunkPosition>>,
}

impl ChunkTickets {
    pub fn insert_ticket(&mut self, chunk: DimensionChunkPosition, ticket: Ticket) {
        self.tickets.entry(chunk.clone()).or_default().push(ticket);
        self.by_entity.entry(ticket).or_default().push(chunk);
    }

    pub fn remove_ticket(&mut self, chunk: &DimensionChunkPosition, ticket: Ticket) {
        if let Some(vec) = self.tickets.get_mut(chunk) {
            if let Some(i) = vec.iter().position(|&e| e == ticket) {
                vec.remove(i);
            }
        }
        if let Some(vec) = self.by_entity.get_mut(&ticket) {
            if let Some(i) = vec.iter().position(|e| e == chunk) {
                vec.remove(i);
            }
        }
    }

    pub fn num_tickets(&self, chunk: &DimensionChunkPosition) -> usize {
        match self.tickets.get(chunk) {
            Some(vec) => vec.len(),
            None => 0,
        }
    }

    pub fn take_tickets(&mut self, ticket: Ticket) -> Vec<DimensionChunkPosition> {
        self.by_entity
            .get_mut(&ticket)
            .map(mem::take)
            .unwrap_or_default()
    }

    pub fn remove_chunk(&mut self, pos: &DimensionChunkPosition) {
        self.tickets.remove(pos);
    }
}

/// ID of a chunk ticket that keeps a chunk loaded.
///
/// Currently just represents an entity, the player
/// that is keeping this chunk loaded.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ticket(pub ChunkLoader);

/// Someone who loads the chunk.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChunkLoader {
    /// A real player.
    Player(Entity),
    /// Not a real player.
    Plugin(Entity),
}

#[derive(Component)]
pub struct PluginChunkLoader;
