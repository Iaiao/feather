//! Chunk loading and unloading based on player `View`s.

use std::time::Instant;

use feather_api::common::chunk::worker::LoadRequest;
use feather_api::common::world::Dimensions;
use feather_api::game::chunk_loading::{ChunkLoadState, ChunkLoader, PluginChunkLoader, Ticket};
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_event::<ChunkLoadEvent>()
        .init_resource::<ChunkLoadState>()
        .add_system(remove_despawned_entities)
        .add_system(update_tickets_for_players)
        .add_system(update_tickets_for_plugins)
        .add_system(unload_chunks)
        .add_system(load_chunks);
}

/// System to populate chunk tickets based on players' views.
fn update_tickets_for_players(
    mut state: ResMut<ChunkLoadState>,
    mut worlds: Query<&mut Dimensions>,
    mut event_reader: EventReader<EntityEvent<ViewUpdateEvent>>,
) {
    for event in event_reader.iter() {
        let mut dimensions = worlds.get_mut(**event.new_world).unwrap();
        let player_ticket = Ticket(ChunkLoader::Player(event.entity));

        // Remove old tickets
        for &old_chunk in &event.old_chunks {
            state.remove_ticket(
                &DimensionChunkPosition::new(
                    *event.old_world,
                    (*event.old_dimension).clone(),
                    old_chunk,
                ),
                player_ticket,
            );
        }

        // Create new tickets
        for &new_chunk in &event.new_chunks {
            state.chunk_tickets.insert_ticket(
                DimensionChunkPosition::new(
                    *event.new_world,
                    (*event.new_dimension).clone(),
                    new_chunk,
                ),
                player_ticket,
            );

            // Load if needed
            let dimension = dimensions.get_mut(&event.new_dimension).unwrap();
            if !dimension.is_chunk_loaded(new_chunk) && !dimension.is_chunk_loading(new_chunk) {
                dimension.queue_chunk_load(LoadRequest { pos: new_chunk });
            }
        }
    }
}

fn update_tickets_for_plugins(
    mut state: ResMut<ChunkLoadState>,
    mut worlds: Query<&mut Dimensions>,
    added: Query<(Entity, &DimensionChunkPosition), Added<PluginChunkLoader>>,
    removed: RemovedComponents<PluginChunkLoader>,
) {
    // Create new tickets
    for (entity, position) in added.iter() {
        let mut dimensions = worlds.get_mut(*position.world()).unwrap();
        let plugin_ticket = Ticket(ChunkLoader::Plugin(entity));
        state
            .chunk_tickets
            .insert_ticket(position.clone(), plugin_ticket);

        // Load if needed
        let dimension = dimensions.get_mut(position.dimension()).unwrap();
        if !dimension.is_chunk_loaded(position.position())
            && !dimension.is_chunk_loading(position.position())
        {
            dimension.queue_chunk_load(LoadRequest {
                pos: position.position(),
            });
        }
    }

    // Remove old tickets
    for entity in removed.iter() {
        state.remove_all_tickets(Ticket(ChunkLoader::Plugin(entity)));
    }
}

/// System to unload chunks from the `ChunkUnloadQueue`.
fn unload_chunks(mut state: ResMut<ChunkLoadState>, mut worlds: Query<&mut Dimensions>) {
    while let Some(unload) = state.chunk_unload_queue.get(0) {
        if unload.unload_at_time > Instant::now() {
            // None of the remaining chunks in the queue are
            // ready for unloading, because the queue is ordered
            // by time.
            break;
        }

        let unload = state.chunk_unload_queue.pop_front().unwrap();

        // If the chunk has acquired new tickets, then abort unloading it.
        if state.chunk_tickets.num_tickets(&unload.pos) > 0 {
            continue;
        }

        log::trace!("Unloading chunk at {:?}", unload.pos);
        if let Err(err) = worlds
            .get_mut(*unload.pos.world())
            .unwrap()
            .get_mut(&unload.pos.dimension())
            .unwrap()
            .unload_chunk(unload.pos.position())
        {
            log::error!("Error unloading chunk at {:?}: {}", unload.pos, err);
            return;
        }
    }
    for mut dimensions in worlds.iter_mut() {
        for dimension in dimensions.iter_mut() {
            if !dimension.cache.is_empty() {
                dimension.cache.purge_old_unused();
            }
        }
    }
}

fn remove_despawned_entities(
    mut state: ResMut<ChunkLoadState>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    for event in event_reader.iter() {
        let entity_ticket = Ticket(ChunkLoader::Player(event.entity));
        state.remove_all_tickets(entity_ticket);
    }
}

/// System to call `World::load_chunks` each tick
fn load_chunks(
    mut worlds: Query<(Entity, &mut Dimensions)>,
    mut event_writer: EventWriter<ChunkLoadEvent>,
) {
    for (world, mut dimensions) in worlds.iter_mut() {
        let world = WorldId(world);
        for dimension in dimensions.iter_mut() {
            let loaded = dimension.load_chunks();
            event_writer.send_batch(loaded.into_iter().map(|pos| ChunkLoadEvent {
                position: pos,
                dimension: dimension.id().clone(),
                world,
            }))
        }
    }
}
