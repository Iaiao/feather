// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::LeashKnot;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct LeashKnotBundle {
    pub marker: LeashKnot,
    #[bundle]
    pub entity: EntityBundle,
}
