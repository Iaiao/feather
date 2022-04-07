use bevy::ecs::bundle::Bundle;

use crate::blocks::BlockId;

use crate::components::entity_markers::FallingBlock;

use super::EntityBundle;

#[derive(Bundle)]
pub struct FallingBlockBundle {
    marker: FallingBlock,
    block: FallingBlockState,
    #[bundle]
    entity: EntityBundle,
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
pub struct FallingBlockState(pub BlockId);
