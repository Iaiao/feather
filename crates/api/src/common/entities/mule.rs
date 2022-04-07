// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Mule;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct MuleBundle {
    pub marker: Mule,
    #[bundle]
    pub entity: EntityBundle,
}
