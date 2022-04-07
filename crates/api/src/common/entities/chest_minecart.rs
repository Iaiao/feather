// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ChestMinecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ChestMinecartBundle {
    pub marker: ChestMinecart,
    #[bundle]
    pub entity: EntityBundle,
}
