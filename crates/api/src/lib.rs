#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::sync::Arc;

pub use bevy;
use flume::Receiver;

use crate::game::chunk_subscriptions::ChunkSubscriptions;
use crate::game::initial_handler::NewPlayer;
use crate::game::listener::Listener;
use crate::game::options::ServerOptions;
use crate::game::player_count::PlayerCount;
use crate::prelude::*;

pub mod base;
pub mod blocks;
pub mod common;
pub mod components;
pub mod core;
pub mod datapacks;
pub mod events;
pub mod game;
pub mod inventory;
pub mod items;
pub mod particles;
pub mod prelude;
pub mod protocol;
pub mod resources;
pub mod text;
pub mod util;
pub mod worldgen;

/// A Minecraft server.
///
/// Call [`link_with_game`](Server::link_with_game) to register the server
/// with a [`Game`](common::Game). This will
/// cause the server to serve the game to players.
///
/// Uses asynchronous IO with Tokio.
pub struct Server {
    clients: Clients,
    new_players: Receiver<NewPlayer>,

    pub waiting_chunks: WaitingChunks,
    pub chunk_subscriptions: ChunkSubscriptions,

    player_count: PlayerCount,
}

impl Server {
    /// Starts a server with the given `Options`.
    ///
    /// Must be called within the context of a Tokio runtime.
    pub async fn bind(options: ServerOptions) -> anyhow::Result<Self> {
        let options = Arc::new(options);
        let player_count = PlayerCount::new(options.max_players);

        let (new_players_tx, new_players) = flume::bounded(4);
        Listener::start(Arc::clone(&options), player_count.clone(), new_players_tx).await?;

        log::info!(
            "Server is listening on {}:{}",
            options.bind_address,
            options.port
        );

        Ok(Self {
            clients: Clients::new(),
            new_players,
            waiting_chunks: WaitingChunks::default(),
            chunk_subscriptions: ChunkSubscriptions::default(),
            player_count,
        })
    }

    /// Gets the number of online players.
    pub fn player_count(&self) -> u32 {
        self.player_count.get()
    }
}

/// Low-level functions, mostly used internally.
/// You may find these useful for some custom functionality.
impl Server {
    /// Polls for newly connected players. Returns the IDs of the new clients.
    pub fn accept_new_players(&mut self) -> Vec<ClientId> {
        let mut clients = Vec::new();
        for player in self.new_players.clone().try_iter() {
            if let Some(old_client) = self.clients.iter_mut().find(|x| x.uuid() == player.uuid) {
                old_client.disconnect("Logged in from another location!");
            }
            let id = self.create_client(player);
            clients.push(id);
        }
        clients
    }

    /// Removes a client.
    pub fn remove_client(&mut self, id: ClientId) {
        let client = self.clients.remove(id);
        if let Some(client) = client {
            log::debug!("Removed client for {}", client.username());
        }
    }

    fn create_client(&mut self, player: NewPlayer) -> ClientId {
        log::debug!("Creating client for {}", player.username);
        let client = Client::new(player);
        self.clients.insert(client)
    }

    /// Invokes a callback on all clients.
    pub fn broadcast_with(&self, mut callback: impl FnMut(&Client)) {
        for client in self.clients.iter() {
            callback(client);
        }
    }

    /// Invokes a callback on all clients.
    pub fn broadcast_with_mut(&mut self, mut callback: impl FnMut(&mut Client)) {
        for client in self.clients.iter_mut() {
            callback(client);
        }
    }

    /// Sends a packet to all clients currently subscribed
    /// to the given position. This function should be
    /// used for entity updates, block updates, etc—
    /// any packets that need to be sent only to nearby players.
    pub fn broadcast_nearby_with(
        &self,
        world: WorldId,
        dimension: DimensionId,
        position: Position,
        mut callback: impl FnMut(&Client),
    ) {
        for client_id in self
            .chunk_subscriptions
            .subscriptions_for(&DimensionChunkPosition::new(
                world,
                dimension,
                position.chunk(),
            ))
        {
            if let Some(client) = self.clients.get(*client_id) {
                callback(client);
            }
        }
    }

    /// Sends a packet to all clients currently subscribed
    /// to the given position. This function should be
    /// used for entity updates, block updates, etc—
    /// any packets that need to be sent only to nearby players.
    pub fn broadcast_nearby_with_mut(
        &mut self,
        world: EntityWorld,
        dimension: EntityDimension,
        position: Position,
        mut callback: impl FnMut(&mut Client),
    ) {
        for client_id in self
            .chunk_subscriptions
            .subscriptions_for(&DimensionChunkPosition::new(
                *world,
                (*dimension).clone(),
                position.chunk(),
            ))
        {
            if let Some(client) = self.clients.get_mut(*client_id) {
                callback(client);
            }
        }
    }

    pub fn broadcast_keepalive(&mut self) {
        self.broadcast_with(|client| client.send_keepalive());
    }

    pub fn client(&self, id: ClientId) -> Option<&Client> {
        self.clients.get(id)
    }

    pub fn client_mut(&mut self, id: ClientId) -> Option<&mut Client> {
        self.clients.get_mut(id)
    }
}
