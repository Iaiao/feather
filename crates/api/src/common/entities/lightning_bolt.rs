// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::LightningBolt;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct LightningBoltBundle {
    pub marker: LightningBolt,
    #[bundle]
    pub entity: EntityBundle,
}