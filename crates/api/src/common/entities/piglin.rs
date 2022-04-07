// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Piglin;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PiglinBundle {
    pub marker: Piglin,
    #[bundle]
    pub entity: EntityBundle,
}
