// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Axolotl;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct AxolotlBundle {
    pub marker: Axolotl,
    #[bundle]
    pub entity: EntityBundle,
}
