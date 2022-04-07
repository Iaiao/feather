use bevy::ecs::prelude::{Entity, Res};
use bevy::ecs::system::SystemState;
use bevy::prelude::{Mut, Query, ResMut};

use interaction::{
    handle_held_item_change, handle_interact_entity, handle_player_block_placement,
    handle_player_digging,
};

use feather_api::components::{
    ChatBox, CreativeFlying, HotbarSlot, OnGround, PlayerName, Sneaking, Sprinting,
};
use feather_api::prelude::Dimensions;
use feather_api::prelude::*;
use feather_api::protocol::{
    packets::{
        client,
        server::{Animation, Hand},
    },
    ClientPlayPacket,
};

mod entity_action;
mod interaction;
pub mod inventory;
mod movement;

/// Handles a packet received from a client.
pub fn handle_packet(player: Entity, world: &mut bevy::prelude::World, packet: ClientPlayPacket) {
    match packet {
        ClientPlayPacket::PlayerPosition(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<(&ClientId, &mut Position, &mut OnGround)>,
            )> = SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (client_id, mut position, mut on_ground) = query.get_mut(player).unwrap();
            movement::handle_player_position(
                &mut *server,
                *client_id,
                &mut *position,
                &mut *on_ground,
                packet,
            )
        }
        ClientPlayPacket::PlayerPositionAndRotation(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<(&ClientId, &mut Position, &mut OnGround)>,
            )> = SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (client_id, mut position, mut on_ground) = query.get_mut(player).unwrap();
            let client = server.client_mut(*client_id).unwrap();
            if client.knows_own_position() {
                movement::handle_player_position_and_rotation(
                    &mut *server,
                    *client_id,
                    &mut *position,
                    &mut *on_ground,
                    packet,
                )
            }
        }
        ClientPlayPacket::PlayerRotation(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<(&ClientId, &mut Position, &mut OnGround)>,
            )> = SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (client_id, mut position, mut on_ground) = query.get_mut(player).unwrap();
            movement::handle_player_rotation(
                &mut *server,
                *client_id,
                &mut *position,
                &mut *on_ground,
                packet,
            )
        }
        ClientPlayPacket::PlayerMovement(packet) => {
            let mut system_state: SystemState<Query<&mut OnGround>> = SystemState::new(world);
            let mut query = system_state.get_mut(world);
            let mut on_ground = query.get_mut(player).unwrap();
            movement::handle_player_movement(&mut *on_ground, packet)
        }

        ClientPlayPacket::Animation(packet) => {
            let mut system_state: SystemState<(
                Res<Server>,
                Query<(&Position, &EntityWorld, &EntityDimension, &NetworkId)>,
            )> = SystemState::new(world);
            let (server, query) = system_state.get(world);
            let (position, world, dimension, network_id) = query.get(player).unwrap();
            handle_animation(&*server, *world, dimension, *position, *network_id, packet)
        }

        ClientPlayPacket::ChatMessage(packet) => {
            let mut system_state: SystemState<(Query<(&PlayerName, &Uuid)>, Query<&mut ChatBox>)> =
                SystemState::new(world);
            let (mut query, mut chatboxes) = system_state.get_mut(world);
            let (name, uuid) = query.get_mut(player).unwrap();
            handle_chat_message(
                name.to_string(),
                *uuid,
                &mut chatboxes.iter_mut().collect::<Vec<_>>(),
                packet,
            )
        }

        ClientPlayPacket::PlayerDigging(packet) => {
            let mut system_state: SystemState<(
                Res<Server>,
                Query<(&HotbarSlot, &ClientId, &mut Window)>,
            )> = SystemState::new(world);
            let (server, mut query) = system_state.get_mut(world);
            let (hotbar_slot, client_id, mut window) = query.get_mut(player).unwrap();
            handle_player_digging(&*server, packet, *hotbar_slot, *client_id, &mut *window)
        }

        ClientPlayPacket::CreativeInventoryAction(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<(&Gamemode, &mut Window, &ClientId)>,
            )> = SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (gamemode, mut window, client_id) = query.get_mut(player).unwrap();
            if let Err(err) = inventory::handle_creative_inventory_action(
                packet,
                &*server,
                *gamemode,
                &mut *window,
                *client_id,
            ) {
                server
                    .client_mut(*client_id)
                    .unwrap()
                    .disconnect(err.to_string())
            }
        }

        ClientPlayPacket::ClickWindow(packet) => {
            let mut system_state: SystemState<(ResMut<Server>, Query<(&mut Window, &ClientId)>)> =
                SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (mut window, client_id) = query.get_mut(player).unwrap();
            if let Err(err) =
                inventory::handle_click_window(&*server, packet, *client_id, &mut *window)
            {
                server
                    .client_mut(*client_id)
                    .unwrap()
                    .disconnect(err.to_string())
            }
        }

        ClientPlayPacket::PlayerBlockPlacement(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<&mut Dimensions>,
                Res<InteractableRegistry>,
                Query<(&ClientId, &EntityDimension, &EntityWorld)>,
                EventWriter<EntityEvent<BlockInteractEvent>>,
                EventWriter<EntityEvent<BlockPlacementEvent>>,
            )> = SystemState::new(world);
            let (
                mut server,
                mut worlds,
                interactable_registry,
                query,
                interaction_writer,
                placement_writer,
            ) = system_state.get_mut(world);
            let (client_id, dimension, world) = query.get(player).unwrap();
            let mut dimensions = worlds.get_mut(***world).unwrap();
            handle_player_block_placement(
                &mut *server,
                packet,
                player,
                *client_id,
                dimensions.get_mut(&*dimension).unwrap(),
                &*interactable_registry,
                interaction_writer,
                placement_writer,
            )
        }

        ClientPlayPacket::HeldItemChange(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<(&mut HotbarSlot, &ClientId)>,
            )> = SystemState::new(world);
            let (mut server, mut query) = system_state.get_mut(world);
            let (mut hotbar_slot, client_id) = query.get_mut(player).unwrap();
            handle_held_item_change(&mut *server, &mut *hotbar_slot, *client_id, packet)
        }

        ClientPlayPacket::InteractEntity(packet) => {
            let mut system_state: SystemState<(
                ResMut<Server>,
                Query<&ClientId>,
                Query<(Entity, &NetworkId)>,
                EventWriter<EntityEvent<InteractEntityEvent>>,
            )> = SystemState::new(world);
            let (mut server, query, entities, event_writer) = system_state.get_mut(world);
            let client_id = query.get(player).unwrap();
            handle_interact_entity(
                &mut *server,
                packet,
                player,
                *client_id,
                &entities.iter().collect::<Vec<_>>(),
                event_writer,
            )
        }

        ClientPlayPacket::ClientSettings(packet) => {
            let mut system_state: SystemState<(Res<Server>, Query<&NetworkId>)> =
                SystemState::new(world);
            let (server, query) = system_state.get(world);
            let network_id = query.get(player).unwrap();
            handle_client_settings(&*server, *network_id, packet)
        }

        ClientPlayPacket::PlayerAbilities(packet) => {
            let mut system_state: SystemState<Query<&mut CreativeFlying>> = SystemState::new(world);
            let mut query = system_state.get_mut(world);
            let mut flying = query.get_mut(player).unwrap();
            movement::handle_player_abilities(&mut *flying, packet)
        }

        ClientPlayPacket::EntityAction(packet) => {
            let mut system_state: SystemState<Query<(&mut Sneaking, &mut Sprinting)>> =
                SystemState::new(world);
            let mut query = system_state.get_mut(world);
            let (mut sneaking, mut sprinting) = query.get_mut(player).unwrap();
            entity_action::handle_entity_action(packet, &mut *sneaking, &mut *sprinting)
        }

        ClientPlayPacket::TeleportConfirm(_)
        | ClientPlayPacket::QueryBlockNbt(_)
        | ClientPlayPacket::SetDifficulty(_)
        | ClientPlayPacket::ClientStatus(_)
        | ClientPlayPacket::TabComplete(_)
        | ClientPlayPacket::ClickWindowButton(_)
        | ClientPlayPacket::CloseWindow(_)
        | ClientPlayPacket::PluginMessage(_)
        | ClientPlayPacket::EditBook(_)
        | ClientPlayPacket::QueryEntityNbt(_)
        | ClientPlayPacket::GenerateStructure(_)
        | ClientPlayPacket::KeepAlive(_)
        | ClientPlayPacket::LockDifficulty(_)
        | ClientPlayPacket::VehicleMove(_)
        | ClientPlayPacket::SteerBoat(_)
        | ClientPlayPacket::PickItem(_)
        | ClientPlayPacket::CraftRecipeRequest(_)
        | ClientPlayPacket::SteerVehicle(_)
        | ClientPlayPacket::SetDisplayedRecipe(_)
        | ClientPlayPacket::SetRecipeBookState(_)
        | ClientPlayPacket::NameItem(_)
        | ClientPlayPacket::ResourcePackStatus(_)
        | ClientPlayPacket::AdvancementTab(_)
        | ClientPlayPacket::SelectTrade(_)
        | ClientPlayPacket::SetBeaconEffect(_)
        | ClientPlayPacket::UpdateCommandBlock(_)
        | ClientPlayPacket::UpdateCommandBlockMinecart(_)
        | ClientPlayPacket::UpdateJigsawBlock(_)
        | ClientPlayPacket::UpdateStructureBlock(_)
        | ClientPlayPacket::UpdateSign(_)
        | ClientPlayPacket::Spectate(_)
        | ClientPlayPacket::UseItem(_)
        | ClientPlayPacket::Pong(_) => (),
    }
}

fn handle_animation(
    server: &Server,
    world: EntityWorld,
    dimension: &EntityDimension,
    pos: Position,
    network_id: NetworkId,
    packet: client::Animation,
) {
    let animation = match packet.hand {
        Hand::Main => Animation::SwingMainArm,
        Hand::Off => Animation::SwingOffhand,
    };

    server.broadcast_nearby_with(*world, (**dimension).clone(), pos, |client| {
        client.send_entity_animation(network_id, animation.clone())
    });
}

fn handle_chat_message(
    player_name: String,
    player_uuid: Uuid,
    chatboxes: &mut [Mut<ChatBox>],
    packet: client::ChatMessage,
) {
    let message = Text::translate_with("chat.type.text", vec![player_name, packet.message]);
    chatboxes
        .iter_mut()
        .for_each(|chat| chat.send_chat(Some(*player_uuid), message.clone()));
}

fn handle_client_settings(server: &Server, network_id: NetworkId, packet: client::ClientSettings) {
    server.broadcast_with(|client| {
        client.send_player_model_flags(network_id, packet.displayed_skin_parts)
    })
}

pub type Disconnect = String;
