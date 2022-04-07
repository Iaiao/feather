// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Boat;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct BoatBundle {
    pub marker: Boat,
    #[bundle]
    pub entity: EntityBundle,
}
