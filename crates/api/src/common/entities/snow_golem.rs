// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::SnowGolem;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SnowGolemBundle {
    pub marker: SnowGolem,
    #[bundle]
    pub entity: EntityBundle,
}
