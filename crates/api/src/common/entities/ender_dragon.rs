// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::EnderDragon;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EnderDragonBundle {
    pub marker: EnderDragon,
    #[bundle]
    pub entity: EntityBundle,
}
