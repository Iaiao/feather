// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Giant;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct GiantBundle {
    pub marker: Giant,
    #[bundle]
    pub entity: EntityBundle,
}
