// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::LlamaSpit;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct LlamaSpitBundle {
    pub marker: LlamaSpit,
    #[bundle]
    pub entity: EntityBundle,
}
