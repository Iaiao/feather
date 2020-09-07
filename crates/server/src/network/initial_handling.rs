//! Initial handling of a connection.

use super::{worker::Worker, NewPlayer};
use base::Text;
use protocol::{
    packets::{
        client::{HandshakeState, LoginStart, Ping, Request},
        server::{Pong, Response},
    },
    ClientHandshakePacket, ServerStatusPacket,
};
use serde::Serialize;
use std::sync::atomic::Ordering;

const SERVER_NAME: &str = "Feather 1.16.2";
const PROTOCOL_VERSION: i32 = 751;

/// Result of initial handling.
pub enum InitialHandling {
    /// The client should be disconnected (sent when
    /// the connection was just a "status" ping.)
    Disconnect,
    /// We should create a new player.
    Join(NewPlayer),
}

/// Handles a connection until the protocol state is switched to Play;
/// that is, until we send Login Success. Returns the client's information.
pub async fn handle(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    // Get the handshake packet.
    let handshake = provider.read::<ClientHandshakePacket>().await?;

    let ClientHandshakePacket::Handshake(handshake) = handshake;

    match handshake.next_state {
        HandshakeState::Status => handle_status(provider).await,
        HandshakeState::Login => handle_login(provider).await,
    }
}

#[derive(Debug, Serialize)]
struct StatusResponse<'a> {
    version: Version,
    players: Players,
    description: &'a Text,
}

#[derive(Debug, Serialize)]
struct Version {
    name: &'static str,
    protocol: i32,
}

#[derive(Debug, Serialize)]
struct Players {
    max: i32,
    online: usize,
}

async fn handle_status(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _request = provider.read::<Request>().await?;

    // TODO: correctly fill in this information.
    let payload = StatusResponse {
        version: Version {
            name: SERVER_NAME,
            protocol: PROTOCOL_VERSION,
        },
        players: Players {
            max: provider.server().config.server.max_players,
            online: provider.server().player_count.load(Ordering::SeqCst),
        },
        description: &provider.server().config.server.motd,
    };
    let response = Response {
        response: serde_json::to_string(&payload)?,
    };
    provider
        .write(&ServerStatusPacket::Response(response))
        .await?;

    if let Ok(ping) = provider.read::<Ping>().await {
        let pong = Pong {
            payload: ping.payload,
        };
        provider.write(&ServerStatusPacket::Pong(pong)).await?;
    }

    Ok(InitialHandling::Disconnect)
}

async fn handle_login(provider: &mut Worker) -> anyhow::Result<InitialHandling> {
    let _login_start = provider.read::<LoginStart>().await?;

    Ok(InitialHandling::Disconnect)
}
