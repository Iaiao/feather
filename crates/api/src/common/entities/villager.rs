// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Villager;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct VillagerBundle {
    pub marker: Villager,
    #[bundle]
    pub entity: EntityBundle,
}
