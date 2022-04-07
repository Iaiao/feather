// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::FishingBobber;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct FishingBobberBundle {
    pub marker: FishingBobber,
    #[bundle]
    pub entity: EntityBundle,
}
