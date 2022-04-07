use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

use ahash::AHashSet;
use flume::{Receiver, Sender};
use itertools::{Either, Itertools};
use slab::Slab;
use uuid::Uuid;

use crate::items::InventorySlot;
use packets::server::{SetSlot, SpawnLivingEntity};

use crate::base::anvil;
use crate::base::block::NetworkBlockPosition;
use crate::base::chunk_lock::ChunkHandle;
use crate::base::metadata::EntityMetadata;
use crate::common::velocity::Velocity;
use crate::common::window::Window;
use crate::common::world::Dimensions;
use crate::components::{ChatKind, ChatMessage, OnGround, PreviousGamemode, ProfileProperty};
use crate::game::initial_handler::NewPlayer;
use crate::prelude::*;
use crate::protocol::packets::server::{
    ChangeGameState, ClearTitles, DimensionCodec, DimensionCodecEntry, DimensionCodecRegistry,
    EntityPosition, EntityPositionAndRotation, EntityTeleport, EntityVelocity, GameStateChange,
    HeldItemChange, MultiBlockChange, PlayerAbilities, Respawn, SetTitleSubtitle, SetTitleText,
    SetTitleTimes, SpawnEntity, SpawnExperienceOrb, SpawnPainting, WindowItems,
};
use crate::protocol::{
    packets::{
        self,
        server::{
            AddPlayer, Animation, BlockChange, ChatPosition, ChunkData, DestroyEntities,
            Disconnect, EntityAnimation, EntityHeadLook, JoinGame, KeepAlive,
            PaintingDirection as ProtocolPaintingDirection, PlayerInfo, PlayerPositionAndLook,
            PluginMessage, SendEntityMetadata, SpawnPlayer, UnloadChunk, UpdateViewPosition,
        },
    },
    ClientPlayPacket, Nbt, ProtocolVersion, ServerPlayPacket, VarInt, VarLong, Writeable,
};

/// Max number of chunks to send to a client per tick.
const MAX_CHUNKS_PER_TICK: usize = 10;

/// ID of a client. Can be reused.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Component)]
pub struct ClientId(usize);

/// Stores all `Client`s.
#[derive(Default)]
pub struct Clients {
    slab: Slab<Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, client: Client) -> ClientId {
        let id = ClientId(self.slab.insert(client));
        self.get_mut(id).unwrap().set_id(id);
        id
    }

    pub fn remove(&mut self, id: ClientId) -> Option<Client> {
        self.slab.try_remove(id.0)
    }

    pub fn get(&self, id: ClientId) -> Option<&Client> {
        self.slab.get(id.0)
    }

    pub fn get_mut(&mut self, id: ClientId) -> Option<&mut Client> {
        self.slab.get_mut(id.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Client> + '_ {
        self.slab.iter().map(|(_i, client)| client)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Client> + '_ {
        self.slab.iter_mut().map(|(_i, client)| client)
    }
}

/// A client connected to a server.
///
/// This struct provides methods to send packets
/// to the client.
pub struct Client {
    packets_to_send: Sender<ServerPlayPacket>,
    received_packets: Receiver<ClientPlayPacket>,
    username: String,
    profile: Vec<ProfileProperty>,
    uuid: Uuid,

    teleport_id_counter: i32,

    network_id: Option<NetworkId>,
    sent_entities: AHashSet<NetworkId>,

    knows_position: bool,
    known_chunks: AHashSet<ChunkPosition>,

    chunk_send_queue: VecDeque<ChunkData>,

    /// The previous own position sent by the client.
    /// Used to detect when we need to teleport the client.
    client_known_position: Option<Position>,

    disconnected: bool,

    dimension: Option<EntityDimension>,
    world: Option<EntityWorld>,

    id: Option<ClientId>,
}

impl Client {
    pub fn new(player: NewPlayer) -> Self {
        Self {
            packets_to_send: player.packets_to_send,
            received_packets: player.received_packets,
            username: player.username,
            teleport_id_counter: 0,
            network_id: None,
            profile: player.profile,
            uuid: player.uuid,
            sent_entities: AHashSet::new(),
            knows_position: false,
            known_chunks: AHashSet::new(),
            chunk_send_queue: VecDeque::new(),
            client_known_position: None,
            disconnected: false,
            dimension: None,
            world: None,
            id: None,
        }
    }

    pub fn set_client_known_position(&mut self, pos: Position) {
        self.client_known_position = Some(pos);
    }

    pub fn client_known_position(&self) -> Option<Position> {
        self.client_known_position
    }

    pub fn profile(&self) -> &[ProfileProperty] {
        &self.profile
    }

    pub fn network_id(&self) -> Option<NetworkId> {
        self.network_id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn received_packets(&self) -> impl Iterator<Item = ClientPlayPacket> + '_ {
        self.received_packets.try_iter()
    }

    pub fn is_disconnected(&self) -> bool {
        self.received_packets.is_disconnected() || self.disconnected
    }

    pub fn known_chunks(&self) -> &AHashSet<ChunkPosition> {
        &self.known_chunks
    }

    pub fn knows_own_position(&self) -> bool {
        self.knows_position
    }

    pub fn tick(&mut self) {
        let num_to_send = MAX_CHUNKS_PER_TICK.min(self.chunk_send_queue.len());
        let packets = self
            .chunk_send_queue
            .drain(..num_to_send)
            .collect::<Vec<_>>();
        for packet in packets {
            log::trace!(
                "Sending chunk at {:?} to {}",
                packet.chunk.as_ref().unwrap_left().read().position(),
                self.username
            );
            self.send_packet(packet);
        }
    }

    /// Returns whether the entity with the given ID
    /// is currently loaded on the client.
    pub fn is_entity_loaded(&self, network_id: NetworkId) -> bool {
        self.sent_entities.contains(&network_id)
    }

    pub fn set_network_id(&mut self, network_id: NetworkId) {
        self.network_id = Some(network_id);
    }

    pub fn send_join_game(
        &mut self,
        gamemode: Gamemode,
        previous_gamemode: PreviousGamemode,
        dimensions: &Dimensions,
        biomes: &BiomeList,
        max_players: i32,
        dimension: EntityDimension,
        world: EntityWorld,
        view_distance: u32,
    ) {
        log::trace!("Sending Join Game to {}", self.username);

        let dimension = dimensions.get(&*dimension).unwrap_or_else(|| {
            panic!("Tried to spawn {} in dimension `{}` but the dimension doesn't exist! Existing dimensions: {}",
                   self.username,
                   **dimension,
                   dimensions.iter().map(|dim| format!("`{}`", *dim.id())).join(", "))
        });
        let dimension_codec = DimensionCodec {
            registries: HashMap::from_iter([
                (
                    "minecraft:dimension_type".to_string(),
                    DimensionCodecRegistry::DimensionType(
                        dimensions
                            .iter()
                            .map(|dim| dim.info())
                            .sorted_by_key(|dim| &dim.r#type)
                            .dedup_by(|a, b| a.r#type == b.r#type)
                            .enumerate()
                            .map(|(i, dim)| DimensionCodecEntry {
                                name: dim.r#type.clone(),
                                id: i as i16,
                                element: dim.info.clone(),
                            })
                            .collect(),
                    ),
                ),
                (
                    "minecraft:worldgen/biome".to_string(),
                    DimensionCodecRegistry::WorldgenBiome(
                        biomes
                            .iter()
                            .enumerate()
                            .map(|(i, (name, biome))| DimensionCodecEntry {
                                name: name.to_owned(),
                                id: i as i16,
                                element: biome.info.clone(),
                            })
                            .collect(),
                    ),
                ),
            ]),
        };

        self.dimension = Some(EntityDimension(DimensionId(dimension.id().to_string())));
        self.world = Some(world);

        self.send_packet(JoinGame {
            entity_id: self.network_id.expect("No network id! Use client.set_network_id(NetworkId) before calling this method.").0,
            is_hardcore: false,
            gamemode,
            previous_gamemode,
            dimension_names: dimensions
                .iter().map(|dim| dim.id().to_string()).collect(),
            dimension_codec: Nbt(dimension_codec),
            dimension: Nbt(dimension.info().info.clone()),
            dimension_name: dimension.id().to_string(),
            hashed_seed: 0,
            max_players,
            view_distance: view_distance as i32,
            simulation_distance: view_distance as i32,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: dimension.is_flat(),
        });
    }

    pub fn send_brand(&self) {
        let mut data = Vec::new();
        "Feather"
            .to_owned()
            .write(&mut data, ProtocolVersion::V1_18_1)
            .unwrap();
        self.send_plugin_message("minecraft:brand", data)
    }

    pub fn send_plugin_message(&self, channel: impl Into<String>, data: impl Into<Vec<u8>>) {
        let channel = channel.into();
        log::trace!("Sending plugin message {} to {}", channel, self.username);
        self.send_packet(PluginMessage {
            channel,
            data: data.into(),
        })
    }

    pub fn update_own_position(&mut self, new_position: Position) {
        log::trace!(
            "Updating position of {} to {:?}",
            self.username,
            new_position
        );
        self.teleport_id_counter += 1;
        self.send_packet(PlayerPositionAndLook {
            x: new_position.x,
            y: new_position.y,
            z: new_position.z,
            yaw: new_position.yaw,
            pitch: new_position.pitch,
            flags: 0,
            teleport_id: self.teleport_id_counter,
            dismount_vehicle: false,
        });
        self.knows_position = true;
        self.client_known_position = Some(new_position);
    }

    pub fn move_to_dimension(
        &mut self,
        dimension: EntityDimension,
        dimensions: &Dimensions,
        gamemode: Gamemode,
        previous_gamemode: PreviousGamemode,
        world: EntityWorld,
    ) {
        let dimension = dimensions.get(&*dimension).unwrap_or_else(|| {
            panic!("Tried to move {} to dimension `{}` but the dimension doesn't exist! Existing dimensions: {}",
                   self.username,
                   **dimension,
                   dimensions.iter().map(|dim| format!("`{}`", *dim.id())).join(", "))
        });
        let dimension_info = dimension.info().info.clone();

        self.dimension = Some(EntityDimension(DimensionId(dimension.id().to_string())));
        self.world = Some(world);

        self.send_packet(Respawn {
            dimension: Nbt(dimension_info),
            dimension_name: dimension.id().to_string(),
            hashed_seed: 0,
            gamemode,
            previous_gamemode,
            is_debug: false,
            is_flat: dimension.is_flat(),
            copy_metadata: true,
        });

        self.knows_position = false;
        self.client_known_position = None;
        self.unload_all_entities();
    }

    pub fn update_own_chunk(&self, pos: ChunkPosition) {
        log::trace!("Updating chunk position of {} to {:?}", self.username, pos);
        self.send_packet(UpdateViewPosition {
            chunk_x: pos.x,
            chunk_z: pos.z,
        });
    }

    pub fn send_chunk(&mut self, chunk: ChunkHandle) {
        self.known_chunks.insert(chunk.read().position());
        self.chunk_send_queue.push_back(ChunkData {
            chunk: Either::Left(chunk),
        });
    }

    pub fn overwrite_chunk(&self, chunk: &ChunkHandle) {
        self.send_packet(ChunkData {
            chunk: Either::Left(Arc::clone(chunk)),
        });
    }

    pub fn send_multi_block_change(
        &self,
        chunk: &ChunkHandle,
        changes: &[(u8, u8, u8)],
        section_y: usize,
    ) {
        let chunk = chunk.read();
        self.send_packet(MultiBlockChange {
            chunk_section_coordinate: ((chunk.position().x as u64 & 0x3FFFFF) << 42)
                | (section_y as u64 & 0xFFFFF)
                | ((chunk.position().z as u64 & 0x3FFFFF) << 20),
            dont_trust_edges: false,
            records: changes
                .iter()
                .map(|(x, y, z)| {
                    VarLong(
                        (chunk
                            .section(section_y)
                            .unwrap()
                            .block_at(*x as usize, *y as usize, *z as usize)
                            .unwrap()
                            .vanilla_id() as i64)
                            << 12
                            | ((*x as i64) << 8 | (*z as i64) << 4 | (*y as i64)),
                    )
                })
                .collect(),
        });
    }

    pub fn send_block_change(&self, position: NetworkBlockPosition, new_block: BlockId) {
        self.send_packet(BlockChange {
            position,
            block: new_block,
        });
    }

    pub fn unload_chunk(&mut self, pos: ChunkPosition) {
        if self.known_chunks.remove(&pos) {
            log::trace!("Unloading chunk at {:?} on {}", pos, self.username);
            self.send_packet(UnloadChunk {
                chunk_x: pos.x,
                chunk_z: pos.z,
            });
        }
    }

    pub fn add_tablist_player(
        &self,
        uuid: Uuid,
        name: String,
        profile: &[ProfileProperty],
        gamemode: Gamemode,
    ) {
        log::trace!("Sending AddPlayer({}) to {}", name, self.username);
        let action = AddPlayer {
            uuid,
            name,
            properties: profile.to_vec(),
            gamemode,
            ping: 0,
            display_name: None,
        };
        self.send_packet(PlayerInfo::AddPlayers(vec![action]));
    }

    pub fn remove_tablist_player(&self, uuid: Uuid) {
        log::trace!("Sending RemovePlayer({}) to {}", uuid, self.username);
        self.send_packet(PlayerInfo::RemovePlayers(vec![uuid]));
    }

    pub fn change_player_tablist_gamemode(&self, uuid: Uuid, gamemode: Gamemode) {
        self.send_packet(PlayerInfo::UpdateGamemodes(vec![(uuid, gamemode)]));
    }

    pub fn unload_entity(&mut self, id: NetworkId) {
        self.unload_entities(&[id])
    }

    pub fn unload_all_entities(&mut self) {
        self.unload_entities(&self.sent_entities.iter().copied().collect_vec())
    }

    pub fn unload_entities(&mut self, ids: &[NetworkId]) {
        if !ids.is_empty() {
            log::trace!("Unloading {:?} on {}", ids, self.username);
            self.sent_entities.retain(|e| !ids.contains(e));
            self.send_packet(DestroyEntities {
                entity_ids: ids.iter().map(|id| VarInt(id.0)).collect(),
            });
        } else {
            log::trace!("Unloading 0 entities on {}", self.username)
        }
    }

    pub fn send_player(&mut self, network_id: NetworkId, uuid: Uuid, pos: Position) {
        log::trace!("Sending {:?} to {}", uuid, self.username);
        assert!(!self.sent_entities.contains(&network_id));
        self.send_packet(SpawnPlayer {
            entity_id: network_id.0,
            player_uuid: uuid,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            yaw: pos.yaw,
            pitch: pos.pitch,
        });
        self.register_entity(network_id);
    }

    pub fn send_living_entity(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        velocity: Velocity,
        pos: Position,
        kind: EntityKind,
    ) {
        log::trace!(
            "Spawning a {:?} on {} (entity type ID: {})",
            kind,
            self.username,
            kind.id()
        );
        self.send_packet(SpawnLivingEntity {
            entity_id: network_id.0,
            uuid,
            kind: kind.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            yaw: pos.yaw,
            pitch: pos.pitch,
            head_pitch: pos.pitch,
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        });
        self.register_entity(network_id);
    }

    pub fn send_experience_orb(
        &mut self,
        network_id: NetworkId,
        pos: Position,
        amount: ExperienceOrbAmount,
    ) {
        self.send_packet(SpawnExperienceOrb {
            entity_id: network_id.0,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            count: *amount,
        });
        self.register_entity(network_id);
    }

    pub fn send_painting(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        painting_type: PaintingType,
        center: NetworkBlockPosition,
        direction: PaintingDirection,
    ) {
        self.send_packet(SpawnPainting {
            entity_id: network_id.0,
            entity_uuid: uuid,
            motive: painting_type.id(),
            location: center,
            direction: match *direction {
                Direction::North => ProtocolPaintingDirection::North,
                Direction::South => ProtocolPaintingDirection::South,
                Direction::West => ProtocolPaintingDirection::West,
                Direction::East => ProtocolPaintingDirection::East,
            },
        });
        self.register_entity(network_id);
    }

    pub fn send_item_frame(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        velocity: Velocity,

        pos: Position,
        facing: ItemFrameFacing,
        kind: EntityKind,
    ) {
        self.send_packet(SpawnEntity {
            entity_id: network_id.0,
            uuid,
            kind: kind.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
            data: match facing.0 {
                FacingCubic::Down => 0,
                FacingCubic::Up => 1,
                FacingCubic::North => 2,
                FacingCubic::South => 3,
                FacingCubic::West => 4,
                FacingCubic::East => 5,
            },
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        });
        self.register_entity(network_id);
    }

    pub fn send_falling_block(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        velocity: Velocity,
        pos: Position,
        state: FallingBlockState,
    ) {
        self.send_packet(SpawnEntity {
            entity_id: network_id.0,
            uuid,
            kind: EntityKind::FallingBlock.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
            data: state.0.vanilla_id() as i32,
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        });
        self.register_entity(network_id);
    }

    pub fn send_projectile(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        velocity: Velocity,
        pos: Position,
        shooter: ProjectileShooter,
        kind: EntityKind,
    ) {
        self.send_packet(SpawnEntity {
            entity_id: network_id.0,
            uuid,
            kind: kind.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
            data: shooter.0 .0,
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        });
        self.register_entity(network_id);
    }

    pub fn send_other_entity(
        &mut self,
        network_id: NetworkId,
        uuid: Uuid,
        velocity: Velocity,
        pos: Position,
        kind: EntityKind,
    ) {
        log::trace!(
            "Spawning a {:?} on {} (entity type ID: {})",
            kind,
            self.username,
            kind.id()
        );
        self.send_packet(SpawnEntity {
            entity_id: network_id.0,
            uuid,
            kind: kind.id() as i32,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            pitch: pos.pitch,
            yaw: pos.yaw,
            data: 0,
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        });
        self.register_entity(network_id);
    }

    pub fn update_entity_velocity(&mut self, network_id: NetworkId, velocity: Velocity) {
        self.send_packet(EntityVelocity {
            entity_id: network_id.0,
            velocity_x: (velocity.x * 8000.0) as i16,
            velocity_y: (velocity.y * 8000.0) as i16,
            velocity_z: (velocity.z * 8000.0) as i16,
        })
    }

    pub fn update_entity_position(
        &mut self,
        network_id: NetworkId,
        position: Position,
        prev_position: PreviousPosition,
        on_ground: OnGround,
        prev_on_ground: PreviousOnGround,
        dimension: &EntityDimension,
        world: EntityWorld,
        dimensions: &Dimensions,
        gamemode: Option<Gamemode>,
        previous_gamemode: Option<PreviousGamemode>,
    ) {
        let another_dimension =
            self.world != Some(world) || self.dimension.as_ref() != Some(dimension);

        if self.network_id == Some(network_id) {
            // This entity is the client. Only update
            // the position if it has changed from the client's
            // known position or dimension/world has changed.
            if another_dimension {
                self.move_to_dimension(
                    dimension.clone(),
                    dimensions,
                    gamemode.unwrap(),
                    previous_gamemode.unwrap(),
                    world,
                );
            } else if Some(position) != self.client_known_position {
                self.update_own_position(position);
            }
        } else if !another_dimension {
            let no_change_yaw = (position.yaw - prev_position.0.yaw).abs() < 0.001;
            let no_change_pitch = (position.pitch - prev_position.0.pitch).abs() < 0.001;

            // If the entity jumps or falls we should send a teleport packet instead to keep relative movement in sync.
            if on_ground != prev_on_ground.0 {
                self.send_packet(EntityTeleport {
                    entity_id: network_id.0,
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    yaw: position.yaw,
                    pitch: position.pitch,
                    on_ground: *on_ground,
                });

                return;
            }

            if no_change_yaw && no_change_pitch {
                self.send_packet(EntityPosition {
                    entity_id: network_id.0,
                    delta_x: ((position.x * 32.0 - prev_position.0.x * 32.0) * 128.0) as i16,
                    delta_y: ((position.y * 32.0 - prev_position.0.y * 32.0) * 128.0) as i16,
                    delta_z: ((position.z * 32.0 - prev_position.0.z * 32.0) * 128.0) as i16,
                    on_ground: on_ground.0,
                });
            } else {
                self.send_packet(EntityPositionAndRotation {
                    entity_id: network_id.0,
                    delta_x: ((position.x * 32.0 - prev_position.0.x * 32.0) * 128.0) as i16,
                    delta_y: ((position.y * 32.0 - prev_position.0.y * 32.0) * 128.0) as i16,
                    delta_z: ((position.z * 32.0 - prev_position.0.z * 32.0) * 128.0) as i16,
                    yaw: position.yaw,
                    pitch: position.pitch,
                    on_ground: on_ground.0,
                });

                // Needed for head orientation
                self.send_packet(EntityHeadLook {
                    entity_id: network_id.0,
                    head_yaw: position.yaw,
                });
            }
        }
    }

    pub fn send_keepalive(&self) {
        log::trace!("Sending keepalive to {}", self.username);
        self.send_packet(KeepAlive { id: 0 });
    }

    pub fn send_entity_animation(&self, network_id: NetworkId, animation: Animation) {
        if self.network_id == Some(network_id) {
            return;
        }
        self.send_packet(EntityAnimation {
            entity_id: network_id.0,
            animation,
        })
    }

    pub fn send_message(&self, message: ChatMessage) {
        let packet = chat_packet(None, message);
        self.send_packet(packet);
    }

    pub fn send_chat_message(&self, sender: Uuid, message: ChatMessage) {
        let packet = chat_packet(Some(sender), message);
        self.send_packet(packet);
    }

    /// Sends all the required packets to display the [`Title`]
    ///
    /// If both the `title` and the `sub_title` are set to `None`
    /// this will call [`Client::hide_title`].
    ///
    /// If `fade_in`, `stay` and `fade_out` are `0`
    /// this will call [`Client::reset_title`].
    pub fn send_title(&self, title: Title) {
        if title.title.is_none() && title.sub_title.is_none() {
            self.hide_title();
        } else if title.fade_in + title.stay + title.fade_out == 0 {
            self.reset_title();
        } else {
            if let Some(main_title) = title.title {
                self.set_title_text(main_title);
            }

            if let Some(sub_title) = title.sub_title {
                self.set_title_subtitle(sub_title)
            }

            self.set_title_times(title.fade_in, title.stay, title.fade_out);
        }
    }

    /// Sets the text of [`Title`] to the specified text
    pub fn set_title_text(&self, title: Text) {
        self.send_packet(SetTitleText {
            title_text: title.to_string(),
        });
    }

    /// Sets the subtitle of [`Title`] to the specified text
    pub fn set_title_subtitle(&self, subtitle: Text) {
        self.send_packet(SetTitleSubtitle {
            subtitle_text: subtitle.to_string(),
        });
    }

    /// Resets the title for the player, this removes
    /// the text from the screen.
    ///
    /// Not to be confused with [`Self::hide_title()`]
    pub fn reset_title(&self) {
        self.send_packet(ClearTitles { reset: true });
    }

    /// Hides the title for the player, this removes
    /// the text from the screen, but it will re-appear again
    /// if the set times packet is sent again.
    ///
    /// Not to be confused with [`Self::reset_title()`]
    pub fn hide_title(&self) {
        self.send_packet(ClearTitles { reset: false });
    }

    /// Sets the time for which the current title will be displayed
    pub fn set_title_times(&self, fade_in: u32, stay: u32, fade_out: u32) {
        self.send_packet(SetTitleTimes {
            fade_in: fade_in as i32,
            stay: stay as i32,
            fade_out: fade_out as i32,
        });
    }

    pub fn send_window_items(&self, window: &Window) {
        log::trace!("Updating inventory for {}", self.username);
        let packet = WindowItems {
            window_id: 0,
            state_id: 0,
            items: window.inner().to_vec(),
            cursor_item: window.cursor_item().clone(),
        };
        self.send_packet(packet);
    }

    pub fn send_inventory_slot(&self, slot: i16, item: &InventorySlot) {
        log::trace!(
            "Setting inventory slot {} of {} to {:?}",
            slot,
            self.username,
            item
        );
        self.send_packet(SetSlot {
            window_id: 0,
            state_id: 0,
            slot,
            slot_data: item.clone(),
        });
    }

    pub fn send_particle(&self, particle: &Particle, long_distance: bool, position: &Position) {
        self.send_packet(packets::server::Particle {
            particle_kind: particle.kind,
            long_distance,
            x: position.x,
            y: position.y,
            z: position.z,
            offset_x: particle.offset_x,
            offset_y: particle.offset_y,
            offset_z: particle.offset_z,
            particle_data: 0.0,
            particle_count: particle.count,
        })
    }

    pub fn send_cursor_slot(&self, item: &InventorySlot) {
        log::trace!("Setting cursor slot of {} to {:?}", self.username, item);
        self.send_inventory_slot(-1, item);
    }

    pub fn send_player_model_flags(&self, network_id: NetworkId, model_flags: u8) {
        let mut entity_metadata = EntityMetadata::new();
        entity_metadata.set(17, model_flags);
        self.send_packet(SendEntityMetadata {
            entity_id: network_id.0,
            entries: entity_metadata,
        });
    }

    pub fn send_entity_metadata(&self, network_id: NetworkId, metadata: EntityMetadata) {
        if self.network_id == Some(network_id) {
            return;
        }
        self.send_packet(SendEntityMetadata {
            entity_id: network_id.0,
            entries: metadata,
        });
    }

    pub fn send_abilities(&self, abilities: &anvil::player::PlayerAbilities) {
        let mut bitfield = 0;
        if *abilities.invulnerable {
            bitfield |= 1 << 0;
        }
        if *abilities.is_flying {
            bitfield |= 1 << 1;
        }
        if *abilities.may_fly {
            bitfield |= 1 << 2;
        }
        if *abilities.instabreak {
            bitfield |= 1 << 3;
        }
        self.send_packet(PlayerAbilities {
            flags: bitfield,
            flying_speed: *abilities.fly_speed,
            fov_modifier: *abilities.walk_speed,
        });
    }

    pub fn send_hotbar_slot(&self, slot: u8) {
        self.send_packet(HeldItemChange { slot });
    }

    pub fn change_gamemode(&self, gamemode: Gamemode) {
        self.send_packet(ChangeGameState {
            state_change: GameStateChange::ChangeGamemode { gamemode },
        })
    }

    fn register_entity(&mut self, network_id: NetworkId) {
        self.sent_entities.insert(network_id);
    }

    fn send_packet(&self, packet: impl Into<ServerPlayPacket>) {
        let packet = packet.into();
        log::trace!("Sending packet #{:02X} to {}", packet.id(), self.username);
        let _ = self.packets_to_send.try_send(packet);
    }

    pub fn disconnect(&mut self, reason: impl Into<Text>) {
        self.disconnected = true;
        self.send_packet(Disconnect {
            reason: reason.into().to_string(),
        });
    }

    pub fn dimension(&self) -> &Option<EntityDimension> {
        &self.dimension
    }

    pub fn world(&self) -> &Option<EntityWorld> {
        &self.world
    }

    pub fn id(&self) -> ClientId {
        self.id.unwrap()
    }

    pub(crate) fn set_id(&mut self, id: ClientId) {
        self.id = Some(id);
    }
}

fn chat_packet(uuid: Option<Uuid>, message: ChatMessage) -> packets::server::ChatMessage {
    packets::server::ChatMessage {
        message: message.text().to_string(),
        position: match message.kind() {
            ChatKind::PlayerChat => ChatPosition::Chat,
            ChatKind::System => ChatPosition::SystemMessage,
            ChatKind::AboveHotbar => ChatPosition::Hotbar,
        },
        sender: uuid.unwrap_or_default(),
    }
}
