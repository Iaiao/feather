// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Spider;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SpiderBundle {
    pub marker: Spider,
    #[bundle]
    pub entity: EntityBundle,
}
