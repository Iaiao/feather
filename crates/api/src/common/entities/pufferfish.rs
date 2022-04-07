// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Pufferfish;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PufferfishBundle {
    pub marker: Pufferfish,
    #[bundle]
    pub entity: EntityBundle,
}
