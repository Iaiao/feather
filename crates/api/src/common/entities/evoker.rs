// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Evoker;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EvokerBundle {
    pub marker: Evoker,
    #[bundle]
    pub entity: EntityBundle,
}
