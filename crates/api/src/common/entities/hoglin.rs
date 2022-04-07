// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Hoglin;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct HoglinBundle {
    pub marker: Hoglin,
    #[bundle]
    pub entity: EntityBundle,
}
