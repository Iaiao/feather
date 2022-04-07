// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Panda;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PandaBundle {
    pub marker: Panda,
    #[bundle]
    pub entity: EntityBundle,
}
