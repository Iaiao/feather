// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::MagmaCube;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct MagmaCubeBundle {
    pub marker: MagmaCube,
    #[bundle]
    pub entity: EntityBundle,
}
