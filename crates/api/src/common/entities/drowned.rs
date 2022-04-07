// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Drowned;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct DrownedBundle {
    pub marker: Drowned,
    #[bundle]
    pub entity: EntityBundle,
}
