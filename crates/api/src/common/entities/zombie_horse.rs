// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ZombieHorse;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ZombieHorseBundle {
    pub marker: ZombieHorse,
    #[bundle]
    pub entity: EntityBundle,
}
