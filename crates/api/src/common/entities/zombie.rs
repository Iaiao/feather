// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Zombie;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ZombieBundle {
    pub marker: Zombie,
    #[bundle]
    pub entity: EntityBundle,
}
