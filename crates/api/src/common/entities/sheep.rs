// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Sheep;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SheepBundle {
    pub marker: Sheep,
    #[bundle]
    pub entity: EntityBundle,
}
