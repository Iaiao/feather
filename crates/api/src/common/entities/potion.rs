// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Potion;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PotionBundle {
    pub marker: Potion,
    #[bundle]
    pub entity: EntityBundle,
}
