// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Parrot;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ParrotBundle {
    pub marker: Parrot,
    #[bundle]
    pub entity: EntityBundle,
}
