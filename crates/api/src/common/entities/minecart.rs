// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Minecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct MinecartBundle {
    pub marker: Minecart,
    #[bundle]
    pub entity: EntityBundle,
}
