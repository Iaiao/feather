// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Skeleton;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SkeletonBundle {
    pub marker: Skeleton,
    #[bundle]
    pub entity: EntityBundle,
}
