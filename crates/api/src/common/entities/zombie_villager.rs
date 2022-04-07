// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ZombieVillager;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ZombieVillagerBundle {
    pub marker: ZombieVillager,
    #[bundle]
    pub entity: EntityBundle,
}
