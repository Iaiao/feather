// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Ravager;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct RavagerBundle {
    pub marker: Ravager,
    #[bundle]
    pub entity: EntityBundle,
}
