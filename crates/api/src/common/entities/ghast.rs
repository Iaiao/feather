// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Ghast;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct GhastBundle {
    pub marker: Ghast,
    #[bundle]
    pub entity: EntityBundle,
}
