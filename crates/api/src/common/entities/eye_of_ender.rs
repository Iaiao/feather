// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::EyeOfEnder;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EyeOfEnderBundle {
    pub marker: EyeOfEnder,
    #[bundle]
    pub entity: EntityBundle,
}
