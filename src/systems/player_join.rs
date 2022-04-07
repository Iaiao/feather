use std::sync::Arc;

use log::debug;

use feather_api::base::anvil::player::PlayerAbilities;
use feather_api::common::velocity::Velocity;
use feather_api::common::world::{Dimensions, WorldPath};
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_event::<EntityEvent<PlayerRespawnEvent>>()
        .add_system_to_stage(CoreStage::PreUpdate, poll_new_players)
        .add_system(send_respawn_packets.after("send_entity_movement"));
}

/// Polls for new clients and sends them the necessary packets
/// to join the game.
fn poll_new_players(
    mut commands: Commands,
    config: Res<Config>,
    worlds: Query<(Entity, &WorldName, &WorldPath, &Dimensions)>,
    biomes: Res<Arc<BiomeList>>,
    mut server: ResMut<Server>,
    mut chatboxes: Query<&mut ChatBox>,
    mut respawn_event_writer: EventWriter<EntityEvent<PlayerRespawnEvent>>,
) {
    let mut chatboxes = chatboxes.iter_mut().collect::<Vec<_>>();
    for client_id in server.accept_new_players() {
        let (world, _, world_path, dimensions) = worlds
            .iter()
            .find(|(_, name, _, _)| ***name == config.worlds.default_world)
            .unwrap();
        accept_new_player(
            &mut commands,
            &mut *server,
            world,
            world_path,
            dimensions,
            &mut chatboxes,
            client_id,
            config.server.default_gamemode,
            config.server.view_distance,
            &**biomes,
            config.server.max_players,
            &mut respawn_event_writer,
            config.server.default_dimension.clone(),
        );
    }
}

fn accept_new_player(
    commands: &mut Commands,
    server: &mut Server,
    world: Entity,
    world_path: &WorldPath,
    dimensions: &Dimensions,
    chatboxes: &mut Vec<Mut<ChatBox>>,
    client_id: ClientId,
    default_gamemode: Gamemode,
    view_distance: u32,
    biomes: &BiomeList,
    max_players: u32,
    respawn_event_writer: &mut EventWriter<EntityEvent<PlayerRespawnEvent>>,
    default_dimension: String,
) {
    let client = server.client_mut(client_id).unwrap();
    let player_data = world_path.load_player_data(client.uuid());
    let dimension = EntityDimension(DimensionId(
        player_data
            .as_ref()
            .map(|data| data.dimension.to_owned())
            .unwrap_or(default_dimension),
    ));
    let position = player_data
        .as_ref()
        .map(|data| Position {
            x: data.animal.base.position[0],
            y: data.animal.base.position[1],
            z: data.animal.base.position[2],
            yaw: data.animal.base.rotation[0],
            pitch: data.animal.base.rotation[1],
        })
        .unwrap_or_default();

    if player_data.is_err() {
        debug!("{} is a new player", client.username())
    }
    let gamemode = player_data
        .as_ref()
        .map(|data| Gamemode::from_id(data.gamemode as u8).expect("Unsupported gamemode"))
        .unwrap_or(default_gamemode);
    let previous_gamemode = player_data
        .as_ref()
        .map(|data| PreviousGamemode::from_id(data.previous_gamemode as i8))
        .unwrap_or_default();

    let network_id = NetworkId::new();
    client.set_network_id(network_id);

    let world = EntityWorld(WorldId(world));
    client.send_join_game(
        gamemode,
        previous_gamemode,
        dimensions,
        biomes,
        max_players as i32,
        dimension.clone(),
        world,
        view_distance,
    );
    client.send_brand();

    // Abilities
    let abilities = player_abilities_or_default(
        player_data.as_ref().map(|data| data.abilities.clone()).ok(),
        gamemode,
    );

    let hotbar_slot = player_data
        .as_ref()
        .map(|data| HotbarSlot::new(data.held_item as usize).unwrap())
        .unwrap_or_else(|_e| HotbarSlot::new(0).unwrap());

    let health = Health(
        player_data
            .as_ref()
            .map(|data| data.animal.health)
            .unwrap_or(20.0),
    );

    let inventory = Inventory::player();
    let window = Window::new(BackingWindow::Player {
        player: inventory.new_handle(),
    });
    if let Ok(data) = player_data {
        for inventory_slot in data.inventory {
            let net_slot = inventory_slot.convert_index();
            let slot = match net_slot {
                Some(slot) => slot,
                None => {
                    log::error!("Failed to convert saved slot into network slot");
                    continue;
                }
            };

            window
                .set_item(
                    slot,
                    InventorySlot::Filled(
                        ItemStack::try_from(inventory_slot)
                            .expect("The player has an invalid item saved in their inventory"),
                    ),
                )
                .unwrap(); // This can't fail since the earlier match filters out all incorrect indexes.
        }
    }

    let player = commands
        .spawn_bundle(PlayerBundle {
            marker: Player,
            window,
            hotbar_slot,
            client_id,
            view: View::new(position.chunk(), view_distance, world, dimension.clone()),
            gamemode,
            previous_gamemode,
            name: PlayerName::new(client.username().to_string()),
            properties: External::new(client.profile().to_vec()),
            chatbox: ChatBox::new(ChatPreference::All),
            sneaking: Sneaking(false),
            sprinting: Sprinting(false),
            abilities,
            living: LivingEntityBundle {
                health,
                inventory,
                custom_name: CustomName(client.username().to_owned()),
                entity: EntityBundle {
                    kind: EntityKind::Player,
                    dimension,
                    world,
                    velocity: Velocity::default_living(),
                    position,
                    previous_position: PreviousPosition(position),
                    on_ground: OnGround(true),
                    previous_on_ground: PreviousOnGround(OnGround(true)),
                    network_id,
                    uuid: External::new(client.uuid()),
                },
            },
        })
        .id();

    respawn_event_writer.send(EntityEvent::new(player, PlayerRespawnEvent));

    broadcast_player_join(chatboxes, client.username())
}

fn broadcast_player_join(chatboxes: &mut [Mut<ChatBox>], username: &str) {
    let message = Text::translate_with("multiplayer.player.joined", vec![username.to_owned()]);
    chatboxes
        .iter_mut()
        .for_each(|chat| chat.send_system(message.clone()))
}

fn player_abilities_or_default(
    data: Option<PlayerAbilities>,
    gamemode: Gamemode,
) -> PlayerAbilities {
    data.unwrap_or(PlayerAbilities {
        walk_speed: WalkSpeed::default(),
        fly_speed: CreativeFlyingSpeed::default(),
        may_fly: CanCreativeFly(matches!(gamemode, Gamemode::Creative | Gamemode::Spectator)),
        is_flying: CreativeFlying(matches!(gamemode, Gamemode::Spectator)),
        may_build: CanBuild(!matches!(gamemode, Gamemode::Adventure)),
        instabreak: Instabreak(matches!(gamemode, Gamemode::Creative)),
        invulnerable: Invulnerable(matches!(gamemode, Gamemode::Creative | Gamemode::Spectator)),
    })
}

fn send_respawn_packets(
    server: Res<Server>,
    query: Query<(
        Entity,
        &ClientId,
        &WalkSpeed,
        &CreativeFlyingSpeed,
        &CanCreativeFly,
        &CreativeFlying,
        &CanBuild,
        &Instabreak,
        &Invulnerable,
        &HotbarSlot,
        &Window,
        &Gamemode,
    )>,
    mut event_reader: EventReader<EntityEvent<PlayerRespawnEvent>>,
    mut event_writer: EventWriter<EntityEvent<GamemodeChangeEvent>>,
) {
    for event in event_reader.iter() {
        let (
            entity,
            &client_id,
            &walk_speed,
            &fly_speed,
            &may_fly,
            &is_flying,
            &may_build,
            &instabreak,
            &invulnerable,
            hotbar_slot,
            window,
            gamemode,
        ) = query.get(event.entity).unwrap();
        let client = server.client(client_id).unwrap();
        client.send_abilities(&PlayerAbilities {
            walk_speed,
            fly_speed,
            may_fly,
            is_flying,
            may_build,
            instabreak,
            invulnerable,
        });
        client.send_hotbar_slot(hotbar_slot.get() as u8);
        client.send_window_items(window);
        event_writer.send(EntityEvent::new(entity, GamemodeChangeEvent(*gamemode)))
    }
}
