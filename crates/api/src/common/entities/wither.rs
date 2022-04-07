// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Wither;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WitherBundle {
    pub marker: Wither,
    #[bundle]
    pub entity: EntityBundle,
}
