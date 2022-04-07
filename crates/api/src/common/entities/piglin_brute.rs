// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::PiglinBrute;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PiglinBruteBundle {
    pub marker: PiglinBrute,
    #[bundle]
    pub entity: EntityBundle,
}
