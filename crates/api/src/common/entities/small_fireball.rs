// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::SmallFireball;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SmallFireballBundle {
    pub marker: SmallFireball,
    #[bundle]
    pub entity: EntityBundle,
}
