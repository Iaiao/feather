// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Stray;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct StrayBundle {
    pub marker: Stray,
    #[bundle]
    pub entity: EntityBundle,
}
