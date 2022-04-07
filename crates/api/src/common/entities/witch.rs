// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Witch;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WitchBundle {
    pub marker: Witch,
    #[bundle]
    pub entity: EntityBundle,
}
