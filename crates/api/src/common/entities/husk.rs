// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Husk;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct HuskBundle {
    pub marker: Husk,
    #[bundle]
    pub entity: EntityBundle,
}
