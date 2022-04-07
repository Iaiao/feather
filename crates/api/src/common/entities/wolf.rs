// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Wolf;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WolfBundle {
    pub marker: Wolf,
    #[bundle]
    pub entity: EntityBundle,
}
