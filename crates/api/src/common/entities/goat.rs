// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Goat;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct GoatBundle {
    pub marker: Goat,
    #[bundle]
    pub entity: EntityBundle,
}
