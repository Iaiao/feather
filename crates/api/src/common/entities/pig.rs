// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Pig;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PigBundle {
    pub marker: Pig,
    #[bundle]
    pub entity: EntityBundle,
}
