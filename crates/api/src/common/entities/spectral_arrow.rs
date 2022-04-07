// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::SpectralArrow;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SpectralArrowBundle {
    pub marker: SpectralArrow,
    #[bundle]
    pub entity: EntityBundle,
}
