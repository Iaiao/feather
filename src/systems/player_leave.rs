use anvil::entity::{AnimalData, BaseEntityData};
use anvil::player::{PlayerAbilities, PlayerData};
use feather_api::base::anvil;
use feather_api::common::world::WorldPath;
use feather_api::prelude::*;
use num_traits::cast::ToPrimitive;

pub fn register(app: &mut App) {
    app.add_system_to_stage(CoreStage::PreUpdate, remove_disconnected_clients);
}

// bevy supports up to 15 components in a single query,
// so we need to split this query into 2 smaller queries
fn remove_disconnected_clients(
    mut server: ResMut<Server>,
    mut chatboxes: Query<&mut ChatBox>,
    worlds: Query<&WorldPath>,
    query: Query<
        (
            Entity,
            &ClientId,
            &PlayerName,
            &Position,
            &Gamemode,
            &PreviousGamemode,
            &Health,
            &HotbarSlot,
            &Inventory,
            &EntityDimension,
            &EntityWorld,
        ),
        (
            With<WalkSpeed>,
            With<CreativeFlyingSpeed>,
            With<CanCreativeFly>,
            With<CreativeFlying>,
            With<CanBuild>,
            With<Instabreak>,
            With<Invulnerable>,
        ),
    >,
    abilities: Query<(
        &WalkSpeed,
        &CreativeFlyingSpeed,
        &CanCreativeFly,
        &CreativeFlying,
        &CanBuild,
        &Instabreak,
        &Invulnerable,
    )>,
    mut event_writer: EventWriter<EntityEvent<EntityRemoveEvent>>,
) {
    let mut chatboxes = chatboxes.iter_mut().collect::<Vec<_>>();
    for (
        player,
        &client_id,
        name,
        position,
        gamemode,
        previous_gamemode,
        health,
        hotbar_slot,
        inventory,
        dimension,
        world,
    ) in query.iter()
    {
        let (walk_speed, fly_speed, can_fly, is_flying, can_build, instabreak, invulnerable) =
            abilities.get(player).unwrap();
        let world_path = worlds.get(***world).unwrap();
        let client = server.client(client_id).unwrap();
        if client.is_disconnected() {
            broadcast_player_leave(&mut chatboxes, name);
            world_path
                .save_player_data(
                    client.uuid(),
                    &create_player_data(
                        *position,
                        *gamemode,
                        *previous_gamemode,
                        *health,
                        PlayerAbilities {
                            walk_speed: *walk_speed,
                            fly_speed: *fly_speed,
                            may_fly: *can_fly,
                            is_flying: *is_flying,
                            may_build: *can_build,
                            instabreak: *instabreak,
                            invulnerable: *invulnerable,
                        },
                        *hotbar_slot,
                        inventory,
                        dimension,
                    ),
                )
                .unwrap_or_else(|e| panic!("Couldn't save data for {}: {}", client.username(), e));

            event_writer.send(EntityEvent::new(player, EntityRemoveEvent));
            server.waiting_chunks.remove_player(player);
        }
    }
}

fn broadcast_player_leave(chatboxes: &mut [Mut<ChatBox>], username: &str) {
    let message = Text::translate_with("multiplayer.player.left", vec![username.to_string()]);
    chatboxes
        .iter_mut()
        .for_each(|chat| chat.send_system(message.clone()))
}

fn create_player_data(
    position: Position,
    gamemode: Gamemode,
    previous_gamemode: PreviousGamemode,
    health: Health,
    abilities: PlayerAbilities,
    hotbar_slot: HotbarSlot,
    inventory: &Inventory,
    dimension: &EntityDimension,
) -> PlayerData {
    PlayerData {
        animal: AnimalData {
            base: BaseEntityData {
                position: [position.x, position.y, position.z].into(),
                rotation: [position.yaw, position.pitch].into(),
                velocity: [0.0, 0.0, 0.0].into(),
            },
            health: *health,
        },
        gamemode: gamemode.to_i32().unwrap(),
        previous_gamemode: previous_gamemode.id() as i32,
        inventory: inventory
            .to_vec()
            .iter()
            .enumerate()
            // Here we filter out all empty slots.
            .filter_map(|(slot, item)| {
                match item {
                    InventorySlot::Filled(item) => {
                        let res = anvil::player::InventorySlot::from_network_index(slot, item);
                        match res {
                            Some(i) => Some(i),
                            None => {
                                log::error!("Failed to convert the slot into anvil format.");
                                None
                            }
                        }
                    }
                    InventorySlot::Empty => {
                        // Empty items are filtered out.
                        None
                    }
                }
            })
            .collect(),
        held_item: hotbar_slot.get() as i32,
        abilities,
        dimension: dimension.to_string(),
    }
}
