use ahash::AHashSet;
use feather_api::common::velocity::Velocity;

use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_event::<SendEntitySpawnPacketEvent>()
        .add_system(update_visible_entities)
        .add_system(send_entities_when_created)
        .add_system(update_entities_on_chunk_cross)
        .add_system_to_stage(CoreStage::PostUpdate, unload_entities_when_removed)
        .add_system_to_stage(CoreStage::PostUpdate, send_player_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_living_entity_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_experience_orb_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_painting_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_item_frame_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_falling_block_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_projectiles_spawn_packet)
        .add_system_to_stage(CoreStage::PostUpdate, send_other_entity_spawn_packet);
}

/// System to spawn entities on clients when they become visible,
/// and despawn entities when they become invisible, based on the client's view.
pub fn update_visible_entities(
    chunk_entities: Res<ChunkEntities>,
    mut server: ResMut<Server>,
    query: Query<(&ClientId, &NetworkId)>,
    mut event_reader: EventReader<EntityEvent<ViewUpdateEvent>>,
    mut event_writer: EventWriter<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter() {
        if let Ok((client_id, network_id)) = query.get(event.entity) {
            let client = match server.client_mut(*client_id) {
                Some(client) => client,
                None => continue,
            };

            // Send newly visible entities
            for &new_chunk in &event.event.new_chunks {
                for &entity in chunk_entities.entities_in_chunk(new_chunk) {
                    if entity != event.entity {
                        event_writer.send(SendEntitySpawnPacketEvent {
                            receiver: *client_id,
                            entity,
                        });
                    }
                }
            }

            // Unload entities no longer visible
            for &old_chunk in &event.event.old_chunks {
                for &entity_id in chunk_entities.entities_in_chunk(old_chunk) {
                    if entity_id != event.entity {
                        client.unload_entity(*network_id);
                    }
                }
            }
        }
    }
}

/// System to send an entity to clients when it is created.
fn send_entities_when_created(
    mut server: ResMut<Server>,
    query: Query<(Entity, &Position, &Uuid, &EntityWorld, &EntityDimension), Added<EntityKind>>,
    mut event_writer: EventWriter<SendEntitySpawnPacketEvent>,
) {
    for (entity, &position, uuid, &world, dimension) in query.iter() {
        server.broadcast_nearby_with_mut(world, dimension.clone(), position, |client| {
            if client.uuid() != **uuid {
                event_writer.send(SendEntitySpawnPacketEvent {
                    receiver: client.id(),
                    entity,
                });
            }
        });
    }
}

/// System to unload an entity on clients when it is removed.
fn unload_entities_when_removed(
    mut server: ResMut<Server>,
    query: Query<(&Position, &NetworkId, &EntityWorld, &EntityDimension)>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    for event in event_reader.iter() {
        let (&position, &network_id, &world, dimension) = query.get(event.entity).unwrap();
        server.broadcast_nearby_with_mut(world, dimension.clone(), position, |client| {
            client.unload_entity(network_id)
        });
    }
}

/// System to send/unsend entities on clients when the entity changes chunks.
fn update_entities_on_chunk_cross(
    mut server: ResMut<Server>,
    query: Query<(Entity, &NetworkId, &Uuid, &EntityWorld, &EntityDimension)>,
    mut event_reader: EventReader<EntityEvent<EntityCrossChunkEvent>>,
    mut event_writer: EventWriter<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter() {
        if let Ok((entity, &network_id, uuid, &world, dimension)) = query.get(event.entity) {
            let old_clients: AHashSet<_> = server
                .chunk_subscriptions
                .subscriptions_for(&DimensionChunkPosition::new(
                    *world,
                    (**dimension).clone(),
                    event.old_chunk,
                ))
                .iter()
                .copied()
                .collect();
            let new_clients: AHashSet<_> = server
                .chunk_subscriptions
                .subscriptions_for(&DimensionChunkPosition::new(
                    *world,
                    (**dimension).clone(),
                    event.new_chunk,
                ))
                .iter()
                .copied()
                .collect();

            for left_client in old_clients.difference(&new_clients) {
                if let Some(client) = server.client_mut(*left_client) {
                    if client.uuid() != **uuid {
                        client.unload_entity(network_id);
                    }
                }
            }

            for send_client in new_clients.difference(&old_clients) {
                if let Some(client) = server.client_mut(*send_client) {
                    if client.uuid() != **uuid {
                        event_writer.send(SendEntitySpawnPacketEvent {
                            receiver: client.id(),
                            entity,
                        });
                    }
                }
            }
        }
    }
}

fn send_player_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Position)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader
        .iter()
        .filter(|event| *kind_query.get(event.entity).unwrap() == EntityKind::Player)
    {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &pos) = query.get(event.entity).unwrap();
        client.send_player(network_id, **uuid, pos);
    }
}

fn send_experience_orb_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Position, &ExperienceOrbAmount)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader
        .iter()
        .filter(|event| *kind_query.get(event.entity).unwrap() == EntityKind::ExperienceOrb)
    {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, &pos, &amount) = query.get(event.entity).unwrap();
        client.send_experience_orb(network_id, pos, amount);
    }
}

fn send_living_entity_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Velocity, &Position)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter().filter(|event| {
        matches!(
            kind_query.get(event.entity).unwrap(),
            // TODO auto-generate .is_living()
            EntityKind::Axolotl
                | EntityKind::Bat
                | EntityKind::Bee
                | EntityKind::Blaze
                | EntityKind::Cat
                | EntityKind::CaveSpider
                | EntityKind::Chicken
                | EntityKind::Cod
                | EntityKind::Cow
                | EntityKind::Creeper
                | EntityKind::Dolphin
                | EntityKind::Donkey
                | EntityKind::Drowned
                | EntityKind::ElderGuardian
                | EntityKind::EnderDragon
                | EntityKind::Enderman
                | EntityKind::Endermite
                | EntityKind::Evoker
                | EntityKind::Fox
                | EntityKind::Ghast
                | EntityKind::Giant
                | EntityKind::GlowSquid
                | EntityKind::Goat
                | EntityKind::Guardian
                | EntityKind::Hoglin
                | EntityKind::Horse
                | EntityKind::Husk
                | EntityKind::Illusioner
                | EntityKind::IronGolem
                | EntityKind::Fireball
                | EntityKind::Llama
                | EntityKind::MagmaCube
                | EntityKind::Mule
                | EntityKind::Mooshroom
                | EntityKind::Ocelot
                | EntityKind::Panda
                | EntityKind::Parrot
                | EntityKind::Phantom
                | EntityKind::Pig
                | EntityKind::Piglin
                | EntityKind::PiglinBrute
                | EntityKind::Pillager
                | EntityKind::PolarBear
                | EntityKind::Pufferfish
                | EntityKind::Rabbit
                | EntityKind::Ravager
                | EntityKind::Salmon
                | EntityKind::Sheep
                | EntityKind::Shulker
                | EntityKind::Silverfish
                | EntityKind::Skeleton
                | EntityKind::SkeletonHorse
                | EntityKind::Slime
                | EntityKind::SnowGolem
                | EntityKind::Spider
                | EntityKind::Squid
                | EntityKind::Stray
                | EntityKind::Strider
                | EntityKind::TraderLlama
                | EntityKind::TropicalFish
                | EntityKind::Turtle
                | EntityKind::Vex
                | EntityKind::Villager
                | EntityKind::Vindicator
                | EntityKind::WanderingTrader
                | EntityKind::Witch
                | EntityKind::Wither
                | EntityKind::WitherSkeleton
                | EntityKind::Wolf
                | EntityKind::Zoglin
                | EntityKind::Zombie
                | EntityKind::ZombieHorse
                | EntityKind::ZombieVillager
                | EntityKind::ZombifiedPiglin
        )
    }) {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &velocity, &pos) = query.get(event.entity).unwrap();
        client.send_living_entity(
            network_id,
            **uuid,
            velocity,
            pos,
            *kind_query.get(event.entity).unwrap(),
        )
    }
}

fn send_painting_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(
        &NetworkId,
        &Uuid,
        &PaintingType,
        &Position,
        &PaintingDirection,
    )>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader
        .iter()
        .filter(|event| *kind_query.get(event.entity).unwrap() == EntityKind::Painting)
    {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &painting_type, &pos, &direction) =
            query.get(event.entity).unwrap();
        client.send_painting(
            network_id,
            **uuid,
            painting_type,
            pos.block().try_into().unwrap(),
            direction,
        );
    }
}

pub fn send_item_frame_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Velocity, &Position, &ItemFrameFacing)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter().filter(|event| {
        matches!(
            kind_query.get(event.entity).unwrap(),
            EntityKind::ItemFrame | EntityKind::GlowItemFrame
        )
    }) {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &velocity, &pos, &facing) = query.get(event.entity).unwrap();
        client.send_item_frame(
            network_id,
            **uuid,
            velocity,
            pos,
            facing,
            *kind_query.get(event.entity).unwrap(),
        );
    }
}

pub fn send_falling_block_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Velocity, &Position, &FallingBlockState)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader
        .iter()
        .filter(|event| *kind_query.get(event.entity).unwrap() == EntityKind::FallingBlock)
    {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &velocity, &pos, &state) = query.get(event.entity).unwrap();
        client.send_falling_block(network_id, **uuid, velocity, pos, state);
    }
}

pub fn send_projectiles_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Velocity, &Position, &ProjectileShooter)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter().filter(|event| {
        matches!(
            kind_query.get(event.entity).unwrap(),
            EntityKind::Arrow
                | EntityKind::DragonFireball
                | EntityKind::Egg
                | EntityKind::Fireball
                | EntityKind::EnderDragon
                | EntityKind::FishingBobber
                | EntityKind::LlamaSpit
                | EntityKind::ShulkerBullet
                | EntityKind::SmallFireball
                | EntityKind::Snowball
                | EntityKind::SpectralArrow
                | EntityKind::Potion
                | EntityKind::ExperienceBottle
                | EntityKind::Trident
                | EntityKind::WitherSkull
        )
    }) {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &velocity, &pos, &shooter) = query.get(event.entity).unwrap();
        client.send_projectile(
            network_id,
            **uuid,
            velocity,
            pos,
            shooter,
            *kind_query.get(event.entity).unwrap(),
        );
    }
}

pub fn send_other_entity_spawn_packet(
    mut server: ResMut<Server>,
    query: Query<(&NetworkId, &Uuid, &Velocity, &Position)>,
    kind_query: Query<&EntityKind>,
    mut event_reader: EventReader<SendEntitySpawnPacketEvent>,
) {
    for event in event_reader.iter().filter(|event| {
        matches!(
            kind_query.get(event.entity).unwrap(),
            EntityKind::AreaEffectCloud
                | EntityKind::ArmorStand
                | EntityKind::Boat
                | EntityKind::EndCrystal
                | EntityKind::EvokerFangs
                | EntityKind::EyeOfEnder
                | EntityKind::Item
                | EntityKind::LeashKnot
                | EntityKind::LightningBolt
                | EntityKind::Marker
                | EntityKind::Minecart
                | EntityKind::ChestMinecart
                | EntityKind::CommandBlockMinecart
                | EntityKind::FurnaceMinecart
                | EntityKind::HopperMinecart
                | EntityKind::SpawnerMinecart
                | EntityKind::TntMinecart
                | EntityKind::Tnt
        )
    }) {
        let client = server.client_mut(event.receiver).unwrap();
        let (&network_id, uuid, &velocity, &pos) = query.get(event.entity).unwrap();
        client.send_other_entity(
            network_id,
            **uuid,
            velocity,
            pos,
            *kind_query.get(event.entity).unwrap(),
        );
    }
}
