use crate::initialhandler::InitialHandlerComponent;
use crate::io::{ServerToListenerMessage, ServerToWorkerMessage};
use crate::prelude::*;
use crate::{add_player, initialhandler as ih, remove_player, Entity, State};
use feather_core::network::packet::{Packet, implementation::*};
use mio_extras::channel::{Receiver, Sender};

//const MAX_KEEP_ALIVE_TIME: u64 = 30;

pub struct NetworkComponent {
    sender: Sender<ServerToWorkerMessage>,
    receiver: Receiver<ServerToWorkerMessage>,
    //last_keep_alive_time: u64,
}

impl NetworkComponent {
    pub fn new(
        sender: Sender<ServerToWorkerMessage>,
        receiver: Receiver<ServerToWorkerMessage>,
    ) -> Self {
        Self { sender, receiver }
    }
}

pub fn network_system(state: &mut State) {
    handle_connections(state);

    send_keep_alives(state);

    poll_for_new_players(state);
}

fn handle_connections(state: &mut State) {
    let mut players_to_remove = vec![];

    for player in state.players.clone() {
        while let Ok(msg) = state.network_components[player].receiver.try_recv() {
            match msg {
                ServerToWorkerMessage::NotifyPacketReceived(packet) => {
                    if let Some(_) = state.ih_components.get(player) {
                        if let Err(e) = ih::handle_packet(state, player, packet) {
                            info!("Disconnecting player: {}", e);
                            ih::disconnect_login(state, player, &e.to_string());
                            remove_player(state, player);
                        }
                    } else {
                        // TODO
                    }
                }
                ServerToWorkerMessage::NotifyDisconnect => {
                    players_to_remove.push(player);
                }
                _ => panic!("Invalid message received from worker thread"),
            }
        }
    }

    for _player in players_to_remove {
        remove_player(state, _player);
    }
}

fn send_keep_alives(state: &mut State) {
    if state.tick_count % TPS != 0 {
        return; // Only run once per second
    }

    for player in state.players.clone() {
        let keep_alive = KeepAliveClientbound::new(0);
        send_packet_to_player(state, player, keep_alive);
    }
}

fn poll_for_new_players(state: &mut State) {
    while let Ok(msg) = state.io_manager.receiver.try_recv() {
        match msg {
            ServerToListenerMessage::NewClient(info) => {
                debug!("Server registering player");
                let player = add_player(state);
                let ih = InitialHandlerComponent::new();
                state.ih_components.set(player, ih);

                let netc = NetworkComponent::new(info.sender, info.receiver);
                state.network_components.set(player, netc);
            }
            _ => panic!("Invalid message received from listener thread"),
        }
    }
}

pub fn send_packet_to_player<P: Packet + 'static>(state: &State, player: Entity, packet: P) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::SendPacket(Box::new(packet)));
}

pub fn enable_compression_for_player(state: &State, player: Entity, threshold: usize) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::EnableCompression(threshold));
}

pub fn enable_encryption_for_player(state: &State, player: Entity, key: [u8; 16]) {
    let comp = &state.network_components[player];
    let _ = comp
        .sender
        .send(ServerToWorkerMessage::EnableEncryption(key));
}

pub fn handle_player_remove(state: &mut State, player: Entity) {
    let comp = &state.network_components[player];
    let _ = comp.sender.send(ServerToWorkerMessage::Disconnect);
}

/// Calculates the relative move fields
/// as used in the EntityRelativeMove packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (u16, u16, u16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as u16;
    let y = ((current.y * 32.0 - old.x * 32.0) * 128.0) as u16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as u16;
    (x, y, z)
}