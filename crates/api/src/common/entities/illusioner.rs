// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Illusioner;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct IllusionerBundle {
    pub marker: Illusioner,
    #[bundle]
    pub entity: EntityBundle,
}
