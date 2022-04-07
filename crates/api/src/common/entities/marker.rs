// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Marker;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct MarkerBundle {
    pub marker: Marker,
    #[bundle]
    pub entity: EntityBundle,
}
