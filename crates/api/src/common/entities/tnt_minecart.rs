// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::TntMinecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TntMinecartBundle {
    pub marker: TntMinecart,
    #[bundle]
    pub entity: EntityBundle,
}
