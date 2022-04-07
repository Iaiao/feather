// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Guardian;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct GuardianBundle {
    pub marker: Guardian,
    #[bundle]
    pub entity: EntityBundle,
}
