use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_event::<EntityEvent<EntityCrossChunkEvent>>()
        .add_system_to_stage(CoreStage::PreUpdate, update_chunk_entities)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            created_entities.label("created_entities"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            removed_entities.after("created_entities"),
        );
}

fn update_chunk_entities(
    mut chunk_entities: ResMut<ChunkEntities>,
    mut query: Query<(Entity, &mut ChunkPosition, &Position)>,
    mut event_writer: EventWriter<EntityEvent<EntityCrossChunkEvent>>,
) {
    // Entities that have crossed chunks
    for (entity, mut old_chunk, &position) in query.iter_mut() {
        let new_chunk = position.chunk();
        if position.chunk() != *old_chunk {
            chunk_entities.update(entity, Some(*old_chunk), new_chunk);
            event_writer.send(EntityEvent::new(
                entity,
                EntityCrossChunkEvent {
                    old_chunk: *old_chunk,
                    new_chunk,
                },
            ));

            *old_chunk = new_chunk;
        }
    }
}

fn created_entities(
    mut commands: Commands,
    mut chunk_entities: ResMut<ChunkEntities>,
    query: Query<(Entity, &Position), Added<EntityKind>>,
) {
    // Entities that have been created
    for (entity, position) in query.iter() {
        let chunk = position.chunk();
        chunk_entities.update(entity, None, chunk);
        commands.entity(entity).insert(chunk);
    }
}

fn removed_entities(
    mut chunk_entities: ResMut<ChunkEntities>,
    query: Query<&ChunkPosition, With<EntityKind>>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    // Entities that have been destroyed
    for event in event_reader.iter() {
        let chunk = *query.get(event.entity).unwrap();
        chunk_entities.remove_entity(event.entity, chunk);
    }
}
