// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Dolphin;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct DolphinBundle {
    pub marker: Dolphin,
    #[bundle]
    pub entity: EntityBundle,
}
