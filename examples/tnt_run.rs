extern crate feather_api;

use std::collections::vec_deque::VecDeque;

use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::Rng;

use feather_api::common::velocity::Velocity;
use feather_api::game::chunk_loading::PluginChunkLoader;
use feather_api::prelude::*;

const PREPARATION_TIME: usize = 5;
const RESULTS_TIME: usize = 5;
const BLOCK_FALL_DELAY: usize = 5;

const FALLING_BLOCK_DESPAWN_Y: f64 = 0.0;
const LOSE_Y: f64 = 5.0;

const SPAWN_CENTER: Position = Position {
    x: 0.0,
    y: 22.0,
    z: 0.0,
    pitch: 0.0,
    yaw: 0.0,
};
const SPAWN_RADIUS: f64 = 10.0;
const LAYER_RADIUS: usize = 15;
const LAYERS: &[i32] = &[20, 15, 10];

// Making it const and without `Lazy` requires `const_mut_refs` feature
static BLOCK_STATE_ID: Lazy<BlockId> = Lazy::new(|| BlockId::tnt());

const SPECTATOR_SPAWN_POINT: Position = Position {
    x: 0.0,
    y: 27.0,
    z: 0.0,
    pitch: 0.0,
    yaw: 90.0,
};

const WORLD_NAME: &str = "world";
const DIMENSION_ID: &str = "minecraft:overworld";

#[derive(DynamicPlugin)]
struct TntRunPlugin;

impl Plugin for TntRunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TntRunState::Starting {
            countdown: PREPARATION_TIME,
        })
        .insert_resource(TntRunChunks({
            let min_chunk_x = ((SPAWN_CENTER.x - SPAWN_RADIUS) / 16.0).floor() as i32;
            let max_chunk_x = ((SPAWN_CENTER.x + SPAWN_RADIUS) / 16.0).floor() as i32;
            let min_chunk_z = ((SPAWN_CENTER.z - SPAWN_RADIUS) / 16.0).floor() as i32;
            let max_chunk_z = ((SPAWN_CENTER.z + SPAWN_RADIUS) / 16.0).floor() as i32;

            let range_x = min_chunk_x..=max_chunk_x;
            let range_z = min_chunk_z..=max_chunk_z;

            range_x
                .cartesian_product(range_z)
                .map(|(x, z)| ChunkPosition::new(x, z))
                .collect()
        }))
        .add_startup_system(startup_system)
        .add_system(arena_fill_startup_system)
        .add_system(player_join_system)
        .add_system(countdown_system)
        .add_system(remove_offline_players_system)
        .add_system(block_queue_system)
        .add_system(block_fall_system)
        .add_system(block_falling_system)
        .add_system(lose_system)
        .add_system(winner_system);
    }
}

fn startup_system(
    mut commands: Commands,
    chunks: Res<TntRunChunks>,
    mut worlds: Query<(Entity, &WorldName, &mut Dimensions)>,
) {
    let (world, _, mut dimensions) = worlds
        .iter_mut()
        .find(|(_, name, _)| ***name == WORLD_NAME)
        .expect("World doesn't exist.");
    let dimension = dimensions
        .get_mut(&DimensionId(DIMENSION_ID.to_owned()))
        .expect("Dimension doesn't exist.");
    let (world, dimension) = (WorldId(world), dimension.id().clone());
    let bundles = chunks.0.clone().into_iter().map(move |pos| {
        (
            PluginChunkLoader,
            DimensionChunkPosition::new(world, dimension.clone(), pos),
        )
    });
    log::info!("Loading {} chunks", bundles.len());
    commands.spawn_batch(bundles);
}

fn arena_fill_startup_system(
    chunks: Res<TntRunChunks>,
    mut worlds: Query<(Entity, &WorldName, &mut Dimensions)>,
    mut event_reader: EventReader<ChunkLoadEvent>,
    mut event_writer: EventWriter<BlockChangeEvent>,
    mut chunks_loaded: Local<Vec<ChunkPosition>>,
    mut done: Local<bool>,
) {
    if !*done {
        let (world, _, mut dimensions) = worlds
            .iter_mut()
            .find(|(_, name, _)| ***name == WORLD_NAME)
            .expect("World doesn't exist.");
        let world = WorldId(world);
        let dimension = dimensions
            .get_mut(&DimensionId(DIMENSION_ID.to_owned()))
            .expect("Dimension doesn't exist.");
        for event in event_reader.iter() {
            if event.world == world && event.dimension == dimension.id() {
                chunks_loaded.push(event.position)
            }
        }
        if chunks.0.iter().all(|c| chunks_loaded.contains(c)) {
            log::info!("Spawn area is loaded");
            regenerate_arena(dimension, world, &mut event_writer);
            chunks_loaded.clear();
            *done = true;
        }
    }
}

fn player_join_system(
    state: Res<TntRunState>,
    mut query: Query<(Entity, &mut Position), Added<Player>>,
    mut event_writer: EventWriter<EntityEvent<GamemodeChangeEvent>>,
) {
    for (player, mut position) in query.iter_mut() {
        match &*state {
            TntRunState::Starting { .. } => {
                respawn_in_arena(player, &mut *position, &mut event_writer)
            }
            TntRunState::Playing { .. } | TntRunState::Waiting { .. } => {
                respawn_as_spectator(player, &mut *position, &mut event_writer);
            }
        }
    }
}

fn countdown_system(
    tick_counter: Res<TickCounter>,
    mut state: ResMut<TntRunState>,
    mut query: Query<(Entity, &mut ChatBox, &mut Position), With<Player>>,
    mut worlds: Query<(Entity, &WorldName, &mut Dimensions)>,
    mut gamemode_event_writer: EventWriter<EntityEvent<GamemodeChangeEvent>>,
    mut block_event_writer: EventWriter<BlockChangeEvent>,
) {
    if **tick_counter % 20 != 0 {
        return;
    }
    match &mut *state {
        TntRunState::Waiting { countdown } => {
            if *countdown == 0 {
                let players = query.iter_mut().collect::<Vec<_>>();
                if !players.is_empty() {
                    *state = TntRunState::Starting {
                        countdown: PREPARATION_TIME,
                    };
                    for (player, _, mut position) in players {
                        respawn_in_arena(player, &mut *position, &mut gamemode_event_writer)
                    }
                    let (world, _, mut dimensions) = worlds
                        .iter_mut()
                        .find(|(_, name, _)| ***name == WORLD_NAME)
                        .expect("World doesn't exist.");
                    let dimension = dimensions
                        .get_mut(&DimensionId(DIMENSION_ID.to_owned()))
                        .expect("Dimension doesn't exist.");
                    regenerate_arena(dimension, WorldId(world), &mut block_event_writer);
                }
            } else {
                *countdown -= 1;
            }
        }
        TntRunState::Starting { countdown } => {
            let mut players = query.iter_mut().collect::<Vec<_>>();
            if players.len() > 1 {
                *countdown -= 1;
                if *countdown == 0 {
                    for (_, ref mut chatbox, _) in players.iter_mut() {
                        chatbox.send_title(Title {
                            title: Some(TextComponent::from("Run!").red().into()),
                            sub_title: None,
                            fade_in: 0,
                            stay: 10,
                            fade_out: 5,
                        });
                    }
                    *state = TntRunState::Playing {
                        block_fall_queue: VecDeque::new(),
                        falling_blocks: Vec::new(),
                        players: players.into_iter().map(|(e, _, _)| e).collect(),
                    };
                } else {
                    for (_, mut chatbox, _) in players {
                        chatbox.send_title(Title {
                            title: Some(TextComponent::from(countdown.to_string()).green().into()),
                            sub_title: Some("Get ready".into()),
                            fade_in: 1,
                            stay: 20,
                            fade_out: 1,
                        });
                    }
                }
            } else {
                if *countdown != PREPARATION_TIME {
                    for (_, mut chatbox, _) in players {
                        chatbox.send_title(Title {
                            title: Some("Preparation interrupted".into()),
                            sub_title: Some("Waiting for other players to join".into()),
                            fade_in: 5,
                            stay: 90,
                            fade_out: 5,
                        });
                    }
                }
                *countdown = PREPARATION_TIME;
            }
        }
        _ => (),
    }
}

fn remove_offline_players_system(
    mut state: ResMut<TntRunState>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    if let TntRunState::Playing { players, .. } = &mut *state {
        let removed_entities = event_reader.iter().map(|e| e.entity).collect::<Vec<_>>();
        players.retain(|p| !removed_entities.contains(p));
    }
}

fn block_queue_system(
    tick_counter: Res<TickCounter>,
    mut state: ResMut<TntRunState>,
    query: Query<(Entity, &Position), With<Player>>,
    mut worlds: Query<(&WorldName, &mut Dimensions)>,
) {
    if let TntRunState::Playing {
        block_fall_queue,
        players,
        ..
    } = &mut *state
    {
        let mut dimensions = worlds
            .iter_mut()
            .find(|(name, _)| ***name == WORLD_NAME)
            .expect("World doesn't exist.")
            .1;
        let dimension = dimensions
            .get_mut(&DimensionId(DIMENSION_ID.to_owned()))
            .expect("Dimension doesn't exist.");
        for (player, &position) in query.iter() {
            if !players.contains(&player) {
                continue;
            }
            let pos1 = position
                + vec3(
                    EntityKind::Player.width() / 2.0,
                    0.0,
                    EntityKind::Player.width() / 2.0,
                );
            let pos2 = position
                + vec3(
                    -EntityKind::Player.width() / 2.0,
                    0.0,
                    EntityKind::Player.width() / 2.0,
                );
            let pos3 = position
                + vec3(
                    EntityKind::Player.width() / 2.0,
                    0.0,
                    -EntityKind::Player.width() / 2.0,
                );
            let pos4 = position
                + vec3(
                    -EntityKind::Player.width() / 2.0,
                    0.0,
                    -EntityKind::Player.width() / 2.0,
                );
            for position in [pos1, pos2, pos3, pos4] {
                let block_pos = position.block().down();
                if dimension.block_at(block_pos) == Some(*BLOCK_STATE_ID)
                    && block_fall_queue
                        .iter()
                        .rfind(|(_, pos)| *pos == block_pos)
                        .is_none()
                {
                    block_fall_queue.push_back((**tick_counter + BLOCK_FALL_DELAY, block_pos));
                }
            }
        }
    }
}

fn block_fall_system(
    tick_counter: Res<TickCounter>,
    mut commands: Commands,
    mut state: ResMut<TntRunState>,
    mut worlds: Query<(Entity, &WorldName, &mut Dimensions)>,
    mut event_writer: EventWriter<BlockChangeEvent>,
) {
    if let TntRunState::Playing {
        block_fall_queue,
        falling_blocks,
        ..
    } = &mut *state
    {
        let count = block_fall_queue
            .iter()
            .position(|(time, _)| *time > **tick_counter)
            .unwrap_or(block_fall_queue.len());
        let (world, _, mut dimensions) = worlds
            .iter_mut()
            .find(|(_, name, _)| ***name == WORLD_NAME)
            .expect("World doesn't exist.");
        let world = WorldId(world);
        let dimension = dimensions
            .get_mut(&DimensionId(DIMENSION_ID.to_owned()))
            .expect("Dimension doesn't exist.");
        for (_, block_pos) in block_fall_queue.drain(..count) {
            set_block_at(
                &mut *dimension,
                world,
                block_pos,
                BlockId::air(),
                &mut event_writer,
            );
            let entity = commands.spawn_bundle(TntBundle {
                marker: Tnt,
                entity: EntityBundle {
                    kind: EntityKind::Tnt,
                    dimension: EntityDimension(DimensionId(DIMENSION_ID.to_owned())),
                    world: EntityWorld(world),
                    velocity: Velocity::default_tnt(),
                    position: block_pos.position(),
                    previous_position: PreviousPosition(block_pos.position()),
                    on_ground: OnGround(true),
                    previous_on_ground: PreviousOnGround(OnGround(true)),
                    network_id: NetworkId::new(),
                    uuid: External::new(uuid::Uuid::new_v4()),
                },
            });
            falling_blocks.push(entity.id());
        }
    }
}

fn block_falling_system(
    mut state: ResMut<TntRunState>,
    mut query: Query<(Entity, &mut Position), With<Tnt>>,
    mut entity_remover: EventWriter<EntityEvent<EntityRemoveEvent>>,
) {
    if let TntRunState::Playing { falling_blocks, .. } = &mut *state {
        for (entity, mut position) in query.iter_mut() {
            if falling_blocks.contains(&entity) {
                position.y -= 0.01;
                if position.y <= FALLING_BLOCK_DESPAWN_Y {
                    entity_remover.send(EntityEvent::new(entity, EntityRemoveEvent));
                    falling_blocks
                        .remove(falling_blocks.iter().position(|e| *e == entity).unwrap());
                }
            }
        }
    }
}

fn lose_system(
    mut state: ResMut<TntRunState>,
    mut query: Query<&mut Position>,
    mut chat: Query<&mut ChatBox>,
    mut event_writer: EventWriter<EntityEvent<GamemodeChangeEvent>>,
) {
    if let TntRunState::Playing { players, .. } = &mut *state {
        let mut lost = Vec::with_capacity(players.len());
        for player in &*players {
            let mut position = query.get_mut(*player).unwrap();
            if position.y <= LOSE_Y {
                chat.get_mut(*player).unwrap().send_title(Title {
                    title: Some(TextComponent::from("You lose").red().into()),
                    sub_title: None,
                    fade_in: 1,
                    stay: 10,
                    fade_out: 8,
                });
                respawn_as_spectator(*player, &mut *position, &mut event_writer);
                lost.push(*player);
            }
        }
        players.retain(|player| !lost.contains(player));
    }
}

fn winner_system(
    mut state: ResMut<TntRunState>,
    mut query: Query<(Entity, &mut ChatBox, &mut Position), With<Player>>,
    names: Query<(Entity, &PlayerName)>,
    mut event_writer: EventWriter<EntityEvent<GamemodeChangeEvent>>,
    mut entity_remover: EventWriter<EntityEvent<EntityRemoveEvent>>,
) {
    if let TntRunState::Playing {
        players,
        falling_blocks,
        ..
    } = &mut *state
    {
        if players.len() == 1 {
            let (winner, winner_name) = names.get(players[0]).unwrap();
            for (entity, mut chatbox, mut position) in query.iter_mut() {
                let text = if entity == winner {
                    respawn_as_spectator(entity, &mut *position, &mut event_writer);
                    TextComponent::from("You won!").green().into()
                } else {
                    TextComponent::from(format!("{} won!", winner_name))
                        .yellow()
                        .into()
                };
                chatbox.send_title(Title {
                    title: Some(text),
                    sub_title: None,
                    fade_in: 1,
                    stay: 70,
                    fade_out: 10,
                });
            }
        } else if players.is_empty() {
            for (_, mut chatbox, _) in query.iter_mut() {
                chatbox.send_title(Title {
                    title: Some(TextComponent::from("Draw!").yellow().into()),
                    sub_title: None,
                    fade_in: 1,
                    stay: 20,
                    fade_out: 10,
                });
            }
        } else {
            return;
        }
        entity_remover.send_batch(
            std::mem::take(falling_blocks)
                .into_iter()
                .map(|e| EntityEvent::new(e, EntityRemoveEvent)),
        );
        *state = TntRunState::Waiting {
            countdown: RESULTS_TIME,
        };
    }
}

enum TntRunState {
    Starting {
        countdown: usize,
    },
    Playing {
        block_fall_queue: VecDeque<(usize, BlockPosition)>,
        falling_blocks: Vec<Entity>,
        players: Vec<Entity>,
    },
    Waiting {
        countdown: usize,
    },
}

struct TntRunChunks(Vec<ChunkPosition>);

fn respawn_as_spectator(
    player: Entity,
    position: &mut Position,
    event_writer: &mut EventWriter<EntityEvent<GamemodeChangeEvent>>,
) {
    event_writer.send(EntityEvent::new(
        player,
        GamemodeChangeEvent(Gamemode::Spectator),
    ));
    *position = SPECTATOR_SPAWN_POINT;
}

fn respawn_in_arena(
    player: Entity,
    position: &mut Position,
    event_writer: &mut EventWriter<EntityEvent<GamemodeChangeEvent>>,
) {
    let mut rng = rand::thread_rng();
    event_writer.send(EntityEvent::new(
        player,
        GamemodeChangeEvent(Gamemode::Adventure),
    ));
    let mut spawn_position = SPAWN_CENTER;
    spawn_position.x += rng.gen_range((-SPAWN_RADIUS)..SPAWN_RADIUS);
    spawn_position.z += rng.gen_range((-SPAWN_RADIUS)..SPAWN_RADIUS);
    *position = spawn_position;
}

fn regenerate_arena(
    dimension: &mut Dimension,
    world: WorldId,
    event_writer: &mut EventWriter<BlockChangeEvent>,
) {
    for x in (-(LAYER_RADIUS as i32))..=(LAYER_RADIUS as i32) {
        for z in (-(LAYER_RADIUS as i32))..=(LAYER_RADIUS as i32) {
            for layer_y in LAYERS {
                set_block_at(
                    dimension,
                    world,
                    BlockPosition::new(
                        x + SPAWN_CENTER.x as i32,
                        *layer_y,
                        z + SPAWN_CENTER.z as i32,
                    ),
                    *BLOCK_STATE_ID,
                    event_writer,
                );
            }
        }
    }
}
