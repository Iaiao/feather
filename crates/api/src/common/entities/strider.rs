// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Strider;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct StriderBundle {
    pub marker: Strider,
    #[bundle]
    pub entity: EntityBundle,
}
