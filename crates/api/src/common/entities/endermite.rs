// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Endermite;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EndermiteBundle {
    pub marker: Endermite,
    #[bundle]
    pub entity: EntityBundle,
}
