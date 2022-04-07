// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::AreaEffectCloud;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct AreaEffectCloudBundle {
    pub marker: AreaEffectCloud,
    #[bundle]
    pub entity: EntityBundle,
}
