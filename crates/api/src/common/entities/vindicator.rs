// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Vindicator;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct VindicatorBundle {
    pub marker: Vindicator,
    #[bundle]
    pub entity: EntityBundle,
}
