//! Sends and unloads entities and chunks for a client.
//!
//! The entities and chunks visible to each client are
//! determined based on the player's [`common::view::View`].
use feather_api::common::world::Dimensions;
use feather_api::prelude::*;

use crate::ParallelSystemDescriptorCoercion;

pub fn register(app: &mut App) {
    app.add_event::<EntityEvent<ViewUpdateEvent>>()
        .add_system(send_new_chunks.before("send_entity_movement"))
        .add_system(send_loaded_chunks)
        .add_system(update_player_views.after("send_entity_movement"))
        .add_system(update_view_on_join);
}

fn send_new_chunks(
    mut server: ResMut<Server>,
    worlds: Query<&Dimensions>,
    query: Query<(&ClientId, &Position, &EntityDimension, &EntityWorld)>,
    mut event_reader: EventReader<EntityEvent<ViewUpdateEvent>>,
) {
    for event in event_reader.iter() {
        let (client_id, position, dimension, world) = query.get(event.entity).unwrap();
        // As ecs removes the client one tick after it gets removed here, it can
        // happen that a client is still listed in the ecs but actually removed here so
        // we need to check if the client is actually still there.
        if let Some(client) = server.client_mut(*client_id) {
            client.update_own_chunk(event.new_view.center());
            let dimensions = worlds.get(***world).unwrap();
            update_chunks(
                event.entity,
                *client_id,
                event,
                *position,
                &mut *server,
                **world,
                dimensions.get(&*dimension).unwrap(),
            );
        }
    }
}

fn update_chunks(
    player: Entity,
    client_id: ClientId,
    event: &ViewUpdateEvent,
    position: Position,
    server: &mut Server,
    world: WorldId,
    dimension: &Dimension,
) {
    // Send chunks that are in the new view but not the old view.
    for &pos in &event.new_chunks {
        if let Some(chunk) = dimension.chunk_map().chunk_handle_at(pos) {
            server.client_mut(client_id).unwrap().send_chunk(chunk);
        } else {
            server.waiting_chunks.insert(
                player,
                DimensionChunkPosition::new(world, dimension.id().clone(), pos),
            );
        }
    }

    // Unsend the chunks that are in the old view but not the new view.
    let client = server.client_mut(client_id).unwrap();
    for pos in &event.old_chunks {
        client.unload_chunk(*pos);
    }

    spawn_client_if_needed(server.client_mut(client_id).unwrap(), position);
}

/// Sends newly loaded chunks to players currently
/// waiting for those chunks to load.
fn send_loaded_chunks(
    mut server: ResMut<Server>,
    query: Query<(&ClientId, &Position)>,
    worlds: Query<&Dimensions>,
    mut event_reader: EventReader<ChunkLoadEvent>,
) {
    for event in event_reader.iter() {
        let dimension = worlds
            .get(*event.world)
            .unwrap()
            .get(&event.dimension)
            .unwrap();
        for player in server
            .waiting_chunks
            .drain_players_waiting_for(DimensionChunkPosition::new(
                event.world,
                event.dimension.clone(),
                event.position,
            ))
        {
            if let Ok((client_id, position)) = query.get(player) {
                if let Some(client) = server.client_mut(*client_id) {
                    client.send_chunk(
                        dimension
                            .chunk_map()
                            .chunk_handle_at(event.position)
                            .unwrap(),
                    );
                    spawn_client_if_needed(client, *position);
                }
            }
        }
    }
}

fn spawn_client_if_needed(client: &mut Client, pos: Position) {
    if !client.knows_own_position() && client.known_chunks().contains(&pos.chunk()) {
        log::debug!("Spawning {}", client.username());
        client.update_own_position(pos);
    }
}

/// Updates players' views when they change chunks.
fn update_player_views(
    mut query: Query<(
        Entity,
        &mut View,
        &Position,
        &PlayerName,
        &EntityDimension,
        &EntityWorld,
    )>,
    mut event_writer: EventWriter<EntityEvent<ViewUpdateEvent>>,
) {
    for (player, mut view, position, name, dimension, world) in query.iter_mut() {
        if position.chunk() != view.center()
            || *dimension != *view.dimension()
            || *world != view.world()
        {
            let new_view = View::new(
                position.chunk(),
                view.view_distance(),
                *world,
                dimension.clone(),
            );

            let event = ViewUpdateEvent::new(&*view, &new_view);
            event_writer.send(EntityEvent::new(player, event));

            *view = new_view;
            log::trace!("View of {} has been updated", name);
        }
    }
}

/// Triggers a ViewUpdateEvent when a player joins the game.
fn update_view_on_join(
    query: Query<(Entity, &View, &PlayerName, &EntityDimension, &EntityWorld), Added<Player>>,
    mut event_writer: EventWriter<EntityEvent<ViewUpdateEvent>>,
) {
    for (entity, view, name, dimension, world) in query.iter() {
        let view_event = ViewUpdateEvent::new(&View::empty(*world, dimension.clone()), view);
        event_writer.send(EntityEvent::new(entity, view_event));
        log::trace!("View of {} has been updated (player joined)", name);
    }
}
