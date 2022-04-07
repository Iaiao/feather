// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::EnderPearl;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EnderPearlBundle {
    pub marker: EnderPearl,
    #[bundle]
    pub entity: EntityBundle,
}
