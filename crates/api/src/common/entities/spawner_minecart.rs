// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::SpawnerMinecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SpawnerMinecartBundle {
    pub marker: SpawnerMinecart,
    #[bundle]
    pub entity: EntityBundle,
}
