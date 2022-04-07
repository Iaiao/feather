// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ExperienceBottle;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ExperienceBottleBundle {
    pub marker: ExperienceBottle,
    #[bundle]
    pub entity: EntityBundle,
}
