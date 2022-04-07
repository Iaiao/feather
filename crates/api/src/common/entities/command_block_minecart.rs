// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::CommandBlockMinecart;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CommandBlockMinecartBundle {
    pub marker: CommandBlockMinecart,
    #[bundle]
    pub entity: EntityBundle,
}
