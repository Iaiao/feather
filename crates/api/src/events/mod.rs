use bevy::ecs::entity::Entity;

pub use block_change::*;
pub use block_interact::*;
pub use chunk_view::*;
pub use entity::*;
pub use interact_entity::*;
pub use particle::*;
pub use plugin_message::*;
use std::ops::{Deref, DerefMut};

mod block_change;
mod block_interact;
mod chunk_view;
mod entity;
mod interact_entity;
mod particle;
mod plugin_message;

/// A temporary struct that represents entity events, which bevy doesn't support yet.
/// When using EventReader<EntityEvent<T>> with queries, you may
/// receive an event before the entity is actually added/updated,
/// so entity events that mutate the entity should be dispatched before they're received
pub struct EntityEvent<T> {
    pub entity: Entity,
    pub event: T,
}

impl<T> EntityEvent<T> {
    pub fn new(entity: Entity, event: T) -> EntityEvent<T> {
        EntityEvent { entity, event }
    }
}

impl<T> Deref for EntityEvent<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

impl<T> DerefMut for EntityEvent<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event
    }
}
