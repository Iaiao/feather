// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Arrow;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ArrowBundle {
    pub marker: Arrow,
    #[bundle]
    pub entity: EntityBundle,
}
