use crate::Server;
use feather_api::components::{CreativeFlying, OnGround};
use feather_api::prelude::ClientId;
use feather_api::prelude::*;
use feather_api::protocol::packets::client::{
    PlayerAbilities, PlayerMovement, PlayerPosition, PlayerPositionAndRotation, PlayerRotation,
};

/// If a player has been teleported by the server,
/// we don't want to override their position if
/// we receive a movement packet before the client
/// is aware of the position update.
fn should_skip_movement(server: &Server, client_id: ClientId, server_position: Position) -> bool {
    match server.client(client_id) {
        Some(client) => client.client_known_position() != Some(server_position),
        None => true,
    }
}

pub fn handle_player_movement(on_ground: &mut OnGround, packet: PlayerMovement) {
    on_ground.0 = packet.on_ground;
}

pub fn handle_player_position(
    server: &mut Server,
    client_id: ClientId,
    pos: &mut Position,
    on_ground: &mut OnGround,
    packet: PlayerPosition,
) {
    if should_skip_movement(server, client_id, *pos) {
        return;
    }
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    on_ground.0 = packet.on_ground;
    update_client_position(server, client_id, *pos);
}

pub fn handle_player_position_and_rotation(
    server: &mut Server,
    client_id: ClientId,
    pos: &mut Position,
    on_ground: &mut OnGround,
    packet: PlayerPositionAndRotation,
) {
    if should_skip_movement(server, client_id, *pos) {
        return;
    }
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    on_ground.0 = packet.on_ground;
    update_client_position(server, client_id, *pos);
}

pub fn handle_player_rotation(
    server: &mut Server,
    client_id: ClientId,
    pos: &mut Position,
    on_ground: &mut OnGround,
    packet: PlayerRotation,
) {
    if should_skip_movement(server, client_id, *pos) {
        return;
    }
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    on_ground.0 = packet.on_ground;
    update_client_position(server, client_id, *pos)
}

fn update_client_position(server: &mut Server, client_id: ClientId, pos: Position) {
    if let Some(client) = server.client_mut(client_id) {
        client.set_client_known_position(pos);
    }
}

/// Handles the PlayerAbilities packet that signals that the client wants to
/// start/stop flying (like in creative mode).
pub fn handle_player_abilities(flying: &mut CreativeFlying, packet: PlayerAbilities) {
    match packet.flags {
        0 => {
            // Flying stopped
            if **flying {
                flying.0 = false;
            }
        }
        2 => {
            // Flying started
            if !**flying {
                flying.0 = true;
            }
        }
        err => {
            log::error!("Got a unexpected flag in the PlayerAbilities packet. The value was: {} and not 0 or 2.", err)
        }
    }
}
