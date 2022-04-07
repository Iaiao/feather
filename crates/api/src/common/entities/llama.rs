// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Llama;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct LlamaBundle {
    pub marker: Llama,
    #[bundle]
    pub entity: EntityBundle,
}
