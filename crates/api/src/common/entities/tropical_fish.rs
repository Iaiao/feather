// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::TropicalFish;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TropicalFishBundle {
    pub marker: TropicalFish,
    #[bundle]
    pub entity: EntityBundle,
}
