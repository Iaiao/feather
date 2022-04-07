// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::DragonFireball;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct DragonFireballBundle {
    pub marker: DragonFireball,
    #[bundle]
    pub entity: EntityBundle,
}
