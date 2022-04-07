// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::TraderLlama;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TraderLlamaBundle {
    pub marker: TraderLlama,
    #[bundle]
    pub entity: EntityBundle,
}
