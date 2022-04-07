// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Trident;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TridentBundle {
    pub marker: Trident,
    #[bundle]
    pub entity: EntityBundle,
}
