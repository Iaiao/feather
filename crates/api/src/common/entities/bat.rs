// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Bat;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct BatBundle {
    pub marker: Bat,
    #[bundle]
    pub entity: EntityBundle,
}
