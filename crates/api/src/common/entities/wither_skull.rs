// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::WitherSkull;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WitherSkullBundle {
    pub marker: WitherSkull,
    #[bundle]
    pub entity: EntityBundle,
}
