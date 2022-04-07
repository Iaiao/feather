use crate::core::Gamemode;
use crate::ClientId;
use bevy::prelude::Entity;
use derive_more::Deref;

/// Triggered when an entity is removed from the world.
///
/// The entity will remain alive until CoreStage::PostUpdate
/// to allow systems to observe this event.
#[derive(Debug)]
pub struct EntityRemoveEvent;

/// Triggered when a player joins, changes dimension and respawns after death
#[derive(Debug)]
pub struct PlayerRespawnEvent;

/// Triggered when a player's gamemode is changed and every time the player respawns.
#[derive(Debug, Deref)]
pub struct GamemodeChangeEvent(pub Gamemode);

/// Triggered when a player receives a spawn packet for the specified [`Entity`].
#[derive(Debug)]
pub struct SendEntitySpawnPacketEvent {
    pub receiver: ClientId,
    pub entity: Entity,
}
