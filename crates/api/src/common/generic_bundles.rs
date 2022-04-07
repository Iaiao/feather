use crate::common::velocity::Velocity;
use crate::prelude::*;

#[derive(Bundle)]
pub struct LivingEntityBundle {
    pub health: Health,
    pub inventory: Inventory,
    pub custom_name: CustomName,
    #[bundle]
    pub entity: EntityBundle,
}

#[derive(Bundle)]
pub struct EntityBundle {
    pub kind: EntityKind,
    pub dimension: EntityDimension,
    pub world: EntityWorld,
    pub velocity: Velocity,
    pub position: Position,
    pub previous_position: PreviousPosition,
    pub on_ground: OnGround,
    pub previous_on_ground: PreviousOnGround,
    pub network_id: NetworkId,
    pub uuid: Uuid,
}

#[derive(Bundle)]
pub struct ProjectileEntityBundle {
    pub shooter: ProjectileShooter,
    #[bundle]
    pub entity: EntityBundle,
}

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Deref,
    derive_more::From,
    bevy::ecs::component::Component,
)]
pub struct ProjectileShooter(pub NetworkId);
