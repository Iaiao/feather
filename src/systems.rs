use bevy::core::FixedTimestep;

use crate::packet_handlers::handle_packet;
use ahash::AHashMap;
use feather_api::bevy::prelude::IntoExclusiveSystem;
use feather_api::prelude::*;
use chunk::{chunk_subscriptions, view};

mod chunk;

mod block;
mod chat;
mod dimensions;
mod entity;
mod entity_physics;
mod gamemode;
mod particle;
mod player_join;
mod player_leave;
mod plugin_message;
mod spawn_packet;
mod tablist;
mod tick_counter;

pub fn register(app: &mut App) {
    app.add_event::<EntityEvent<BlockInteractEvent>>()
        .add_event::<EntityEvent<BlockPlacementEvent>>()
        .add_event::<EntityEvent<InteractEntityEvent>>()
        .init_resource::<InteractableRegistry>()
        .add_system_to_stage(CoreStage::PreUpdate, handle_packets.exclusive_system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(5.0))
                .with_system(send_keepalives),
        );

    block::register(app);
    chat::register(app);
    chunk_subscriptions::register(app);
    chunk::loading::register(app);
    chunk::entities::register(app);
    dimensions::register(app);
    entity::register(app);
    entity_physics::register(app);
    gamemode::register(app);
    particle::register(app);
    player_leave::register(app);
    player_join::register(app);
    plugin_message::register(app);
    tablist::register(app);
    tick_counter::register(app);
    view::register(app);

    // other events, often used by plugins
    app.add_event::<BlockChangeEvent>()
        .add_event::<EntityEvent<PluginMessageEvent>>()
        .add_event::<ParticleEvent>();

    app.add_system(tick_clients);
}

/// Polls for packets received from clients
/// and handles them.
fn handle_packets(world: &mut bevy::prelude::World) {
    let mut packets = AHashMap::new();
    for (player, client_id) in world
        .query_filtered::<(Entity, &ClientId), With<Player>>()
        .iter(world)
    {
        let server = world.get_resource::<Server>().unwrap();
        if let Some(client) = server.client(*client_id) {
            packets.insert(player, Vec::new());
            for packet in client.received_packets() {
                log::trace!(
                    "Received packet #{:02X} from {}",
                    packet.id(),
                    client.username()
                );
                packets.get_mut(&player).unwrap().push(packet)
            }
        }
    }
    for (player, packets) in packets {
        for packet in packets {
            handle_packet(player, world, packet)
        }
    }
}

/// Sends out keepalive packets at an interval.
fn send_keepalives(mut server: ResMut<Server>) {
    server.broadcast_keepalive();
}

/// Ticks `Client`s.
fn tick_clients(mut server: ResMut<Server>) {
    server.broadcast_with_mut(Client::tick);
}
