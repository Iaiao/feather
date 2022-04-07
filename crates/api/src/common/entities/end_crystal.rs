// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::EndCrystal;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EndCrystalBundle {
    pub marker: EndCrystal,
    #[bundle]
    pub entity: EntityBundle,
}
