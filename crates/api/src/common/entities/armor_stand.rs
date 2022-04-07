// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ArmorStand;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ArmorStandBundle {
    pub marker: ArmorStand,
    #[bundle]
    pub entity: EntityBundle,
}
