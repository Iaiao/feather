// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Tnt;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TntBundle {
    pub marker: Tnt,
    #[bundle]
    pub entity: EntityBundle,
}
