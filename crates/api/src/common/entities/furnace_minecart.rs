// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::FurnaceMinecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct FurnaceMinecartBundle {
    pub marker: FurnaceMinecart,
    #[bundle]
    pub entity: EntityBundle,
}
