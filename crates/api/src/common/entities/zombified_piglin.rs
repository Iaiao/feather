// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ZombifiedPiglin;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ZombifiedPiglinBundle {
    pub marker: ZombifiedPiglin,
    #[bundle]
    pub entity: EntityBundle,
}
