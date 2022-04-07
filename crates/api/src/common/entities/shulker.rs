// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Shulker;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ShulkerBundle {
    pub marker: Shulker,
    #[bundle]
    pub entity: EntityBundle,
}
