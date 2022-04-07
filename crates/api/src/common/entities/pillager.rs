// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Pillager;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PillagerBundle {
    pub marker: Pillager,
    #[bundle]
    pub entity: EntityBundle,
}
