// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Vex;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct VexBundle {
    pub marker: Vex,
    #[bundle]
    pub entity: EntityBundle,
}
