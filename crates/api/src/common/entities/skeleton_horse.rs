// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::SkeletonHorse;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SkeletonHorseBundle {
    pub marker: SkeletonHorse,
    #[bundle]
    pub entity: EntityBundle,
}
