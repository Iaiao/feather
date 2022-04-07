// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Fox;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct FoxBundle {
    pub marker: Fox,
    #[bundle]
    pub entity: EntityBundle,
}
