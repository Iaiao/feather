use crate::bevy::app::EventWriter;
use crate::bevy::ecs::entity::Entity;
use feather_api::prelude::*;
use feather_api::protocol::packets::client::{
    HeldItemChange, InteractEntity, InteractEntityKind, PlayerBlockPlacement, PlayerDigging,
    PlayerDiggingStatus,
};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    server: &mut Server,
    packet: PlayerBlockPlacement,
    player: Entity,
    client_id: ClientId,
    world: &mut Dimension,
    interactable_registry: &InteractableRegistry,
    mut interaction_event_writer: EventWriter<EntityEvent<BlockInteractEvent>>,
    mut placement_event_writer: EventWriter<EntityEvent<BlockPlacementEvent>>,
) {
    let hand = match packet.hand {
        0 => Hand::Main,
        1 => Hand::Offhand,
        _ => {
            let client = server.client_mut(client_id).unwrap();

            client.disconnect("Malformed Packet!");

            log::info!(
                "Player sent a malformed `PlayerBlockPlacement` packet. {:?}",
                packet
            );

            return;
        }
    };

    let face = match packet.face {
        feather_api::protocol::packets::client::BlockFace::North => {
            feather_api::prelude::BlockFace::North
        }
        feather_api::protocol::packets::client::BlockFace::South => {
            feather_api::prelude::BlockFace::South
        }
        feather_api::protocol::packets::client::BlockFace::East => {
            feather_api::prelude::BlockFace::East
        }
        feather_api::protocol::packets::client::BlockFace::West => {
            feather_api::prelude::BlockFace::West
        }
        feather_api::protocol::packets::client::BlockFace::Top => {
            feather_api::prelude::BlockFace::Top
        }
        feather_api::protocol::packets::client::BlockFace::Bottom => {
            feather_api::prelude::BlockFace::Bottom
        }
    };

    let cursor_position = Vec3f::new(
        packet.cursor_position_x,
        packet.cursor_position_y,
        packet.cursor_position_z,
    );

    let block_kind = {
        let result = world.block_at(packet.position.into());
        match result {
            Some(block) => block.kind(),
            None => {
                let client = server.client_mut(client_id).unwrap();

                client.disconnect("Attempted to interact with an unloaded block!");

                log::info!(
                    "Player attempted to interact with an unloaded block. {:?}",
                    packet
                );

                return;
            }
        }
    };

    if interactable_registry.is_registered(block_kind) {
        // Handle this as a block interaction
        let event = BlockInteractEvent {
            hand,
            location: packet.position.into(),
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        interaction_event_writer.send(EntityEvent::new(player, event));
    } else {
        // Handle this as a block placement
        let event = BlockPlacementEvent {
            hand,
            location: packet.position.into(),
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        placement_event_writer.send(EntityEvent::new(player, event));
    }
}

/// Handles the Player Digging packet sent for the following
/// actions:
/// * Breaking blocks.
/// * Dropping items.
/// * Shooting arrows.
/// * Eating.
/// * Swapping items between the main and off hand.
pub fn handle_player_digging(
    server: &Server,
    packet: PlayerDigging,
    hotbar_slot: HotbarSlot,
    client_id: ClientId,
    window: &mut Window,
) {
    log::trace!("Got player digging with status {:?}", packet.status);
    match packet.status {
        PlayerDiggingStatus::StartDigging | PlayerDiggingStatus::CancelDigging => {
            // TODO
        }
        PlayerDiggingStatus::SwapItemInHand => {
            let hotbar_index = SLOT_HOTBAR_OFFSET + hotbar_slot.get();
            let offhand_index = SLOT_OFFHAND;

            {
                let mut hotbar_item = window.item(hotbar_index).unwrap();
                let mut offhand_item = window.item(offhand_index).unwrap();

                std::mem::swap(&mut *hotbar_item, &mut *offhand_item);
            }

            let client = server.client(client_id).unwrap();

            client.send_window_items(window);
        }
        _ => (),
    }
}

pub fn handle_interact_entity(
    server: &mut Server,
    packet: InteractEntity,
    player: Entity,
    client_id: ClientId,
    entities: &[(Entity, &NetworkId)],
    mut event_writer: EventWriter<EntityEvent<InteractEntityEvent>>,
) {
    let target = {
        let mut found_entity: Option<Entity> = None;
        for (entity, &network_id) in entities.iter() {
            if network_id.0 == packet.entity_id {
                found_entity = Some(*entity);
                break;
            }
        }

        match found_entity {
            None => {
                let client = server.client_mut(client_id).unwrap();

                client.disconnect("Interacted with an invalid entity!");

                log::info!("Player attempted to interact with an invalid entity.");

                return;
            }
            Some(entity) => entity,
        }
    };

    let event = match packet.kind {
        InteractEntityKind::Attack => InteractEntityEvent {
            target,
            ty: InteractionType::Attack,
            target_pos: None,
            hand: None,
            sneaking: packet.sneaking,
        },
        InteractEntityKind::Interact => InteractEntityEvent {
            target,
            ty: InteractionType::Interact,
            target_pos: None,
            hand: None,
            sneaking: packet.sneaking,
        },
        InteractEntityKind::InteractAt {
            target_x,
            target_y,
            target_z,
            hand,
        } => {
            let hand = match hand {
                0 => Hand::Main,
                1 => Hand::Offhand,
                _ => unreachable!(),
            };

            InteractEntityEvent {
                target,
                ty: InteractionType::Attack,
                target_pos: Some(Vec3f::new(
                    target_x as f32,
                    target_y as f32,
                    target_z as f32,
                )),
                hand: Some(hand),
                sneaking: packet.sneaking,
            }
        }
    };

    event_writer.send(EntityEvent::new(player, event));
}

pub fn handle_held_item_change(
    server: &mut Server,
    slot: &mut HotbarSlot,
    client_id: ClientId,
    packet: HeldItemChange,
) {
    log::trace!(
        "Got player slot change from {} to {}",
        slot.get(),
        packet.slot
    );

    if slot.set(packet.slot as usize).is_err() {
        server
            .client_mut(client_id)
            .unwrap()
            .disconnect("Invalid hotbar slot!");
        log::info!(
            "Player attempted to change their hotbar slot to {}",
            packet.slot
        );
    }
}
