// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Enderman;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EndermanBundle {
    pub marker: Enderman,
    #[bundle]
    pub entity: EntityBundle,
}
