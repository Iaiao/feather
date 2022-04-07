// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Zoglin;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ZoglinBundle {
    pub marker: Zoglin,
    #[bundle]
    pub entity: EntityBundle,
}
