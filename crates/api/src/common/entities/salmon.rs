// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Salmon;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SalmonBundle {
    pub marker: Salmon,
    #[bundle]
    pub entity: EntityBundle,
}
