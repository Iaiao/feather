//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.
use feather_api::base::metadata::{
    EntityBitMask, EntityMetadata, Pose, META_INDEX_ENTITY_BITMASK, META_INDEX_POSE,
};
use feather_api::common::velocity::Velocity;
use feather_api::common::world::Dimensions;
use feather_api::prelude::*;

use crate::systems::spawn_packet;
use crate::ParallelSystemDescriptorCoercion;

pub fn register(app: &mut App) {
    spawn_packet::register(app);
    app.add_event::<EntityEvent<EntityRemoveEvent>>()
        .add_system(send_entity_velocity.before("send_entity_movement"))
        .add_system(send_entity_movement.label("send_entity_movement"))
        .add_system(send_entity_sneak_sprint_metadata)
        .add_stage_after(CoreStage::Last, "TODOlast", SystemStage::parallel())
        .add_system_to_stage("TODOlast", despawn_removed_entities)
        .add_system_to_stage("TODOlast", remove_clients);
}

/// Sends entity movement packets.
fn send_entity_movement(
    mut server: ResMut<Server>,
    mut query: Query<
        (
            &Position,
            &mut PreviousPosition,
            &OnGround,
            &NetworkId,
            &mut PreviousOnGround,
            &EntityDimension,
            &EntityWorld,
            Option<&Gamemode>,
            Option<&PreviousGamemode>,
        ),
        Or<(
            Changed<Position>,
            Changed<EntityDimension>,
            Changed<EntityWorld>,
        )>,
    >,
    worlds: Query<(Entity, &Dimensions)>,
) {
    for (
        &position,
        mut prev_position,
        &on_ground,
        &network_id,
        mut prev_on_ground,
        dimension,
        &world,
        gamemode,
        previous_gamemode,
    ) in query.iter_mut()
    {
        let (_, dimensions) = worlds.iter().find(|(w, _)| *w == **world).unwrap();
        server.broadcast_nearby_with_mut(world, dimension.clone(), position, |client| {
            client.update_entity_position(
                network_id,
                position,
                *prev_position,
                on_ground,
                *prev_on_ground,
                dimension,
                world,
                dimensions,
                gamemode.copied(),
                previous_gamemode.copied(),
            );
        });
        prev_position.0 = position;
        if on_ground != prev_on_ground.0 {
            prev_on_ground.0 = on_ground;
        }
    }
}

/// Sends entity velocity packets.
fn send_entity_velocity(
    mut server: ResMut<Server>,
    mut query: Query<
        (
            &Velocity,
            &Position,
            &NetworkId,
            &EntityWorld,
            &EntityDimension,
        ),
        Changed<Velocity>,
    >,
) {
    for (&velocity, &position, &network_id, &world, dimension) in query.iter_mut() {
        server.broadcast_nearby_with_mut(world, dimension.clone(), position, |client| {
            client.update_entity_velocity(network_id, velocity);
        });
    }
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sneaking or sprinting.
fn send_entity_sneak_sprint_metadata(
    server: Res<Server>,
    query: Query<
        (
            &Position,
            &Sneaking,
            &Sprinting,
            &NetworkId,
            &EntityWorld,
            &EntityDimension,
        ),
        Or<(Changed<Sneaking>, Changed<Sprinting>)>,
    >,
) {
    for (&position, &is_sneaking, &is_sprinting, &network_id, &world, dimension) in query.iter() {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        bit_mask.set(EntityBitMask::CROUCHED, *is_sneaking);
        bit_mask.set(EntityBitMask::SPRINTING, *is_sprinting);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        if *is_sneaking {
            metadata.set(META_INDEX_POSE, Pose::Sneaking);
        } else {
            metadata.set(META_INDEX_POSE, Pose::Standing);
        }

        server.broadcast_nearby_with(*world, (**dimension).clone(), position, |client| {
            client.send_entity_metadata(network_id, metadata.clone());
        });
    }
}

fn despawn_removed_entities(
    mut commands: Commands,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    event_reader
        .iter()
        .for_each(|event| commands.entity(event.entity).despawn())
}

fn remove_clients(
    mut server: ResMut<Server>,
    query: Query<&ClientId>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    for event in event_reader.iter() {
        if let Ok(&client_id) = query.get(event.entity) {
            server.remove_client(client_id);
        }
    }
}
