// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::EvokerFangs;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EvokerFangsBundle {
    pub marker: EvokerFangs,
    #[bundle]
    pub entity: EntityBundle,
}
