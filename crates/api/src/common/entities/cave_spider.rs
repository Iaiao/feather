// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::CaveSpider;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CaveSpiderBundle {
    pub marker: CaveSpider,
    #[bundle]
    pub entity: EntityBundle,
}
