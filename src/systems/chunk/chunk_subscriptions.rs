use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(update_chunk_subscriptions)
        .add_system(remove_chunks_with_no_subscriptions);
}

fn update_chunk_subscriptions(
    mut server: ResMut<Server>,
    query: Query<(&ClientId, &View, &EntityWorld, &EntityDimension)>,
    mut update_event_reader: EventReader<EntityEvent<ViewUpdateEvent>>,
    mut remove_event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    // Update players whose views have changed
    for event in update_event_reader.iter() {
        if let Ok((client_id, _, _, _)) = query.get(event.entity) {
            for new_chunk in event.new_view.difference(&event.old_view) {
                add_subscription(
                    &mut *server,
                    DimensionChunkPosition::new(
                        *event.new_view.world(),
                        (**event.new_view.dimension()).clone(),
                        new_chunk,
                    ),
                    *client_id,
                )
            }
            for old_chunk in event.old_view.difference(&event.new_view) {
                remove_subscription(
                    &mut *server,
                    DimensionChunkPosition::new(
                        *event.old_view.world(),
                        (**event.old_view.dimension()).clone(),
                        old_chunk,
                    ),
                    *client_id,
                );
            }
        }
    }

    // Update players that have left
    for event in remove_event_reader.iter() {
        if let Ok((client_id, view, &world, dimension)) = query.get(event.entity) {
            for chunk in view.iter() {
                remove_subscription(
                    &mut *server,
                    DimensionChunkPosition::new(*world, (**dimension).clone(), chunk),
                    *client_id,
                );
            }
        }
    }
}

fn remove_subscription(server: &mut Server, chunk: DimensionChunkPosition, loader: ClientId) {
    if let Some(vec) = server.chunk_subscriptions.chunks.get_mut(&chunk) {
        vec.remove(vec.iter().position(|&item| item == loader).unwrap());
    }
}

fn add_subscription(server: &mut Server, chunk: DimensionChunkPosition, loader: ClientId) {
    server
        .chunk_subscriptions
        .chunks
        .entry(chunk)
        .or_default()
        .push(loader)
}

fn remove_chunks_with_no_subscriptions(mut server: ResMut<Server>) {
    server
        .chunk_subscriptions
        .chunks
        .retain(|_, subscriptions| !subscriptions.is_empty());
}
