// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Silverfish;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SilverfishBundle {
    pub marker: Silverfish,
    #[bundle]
    pub entity: EntityBundle,
}
