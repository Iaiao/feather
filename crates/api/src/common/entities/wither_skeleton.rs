// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::WitherSkeleton;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WitherSkeletonBundle {
    pub marker: WitherSkeleton,
    #[bundle]
    pub entity: EntityBundle,
}
