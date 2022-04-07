// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Creeper;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CreeperBundle {
    pub marker: Creeper,
    #[bundle]
    pub entity: EntityBundle,
}
