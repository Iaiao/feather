// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Ocelot;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct OcelotBundle {
    pub marker: Ocelot,
    #[bundle]
    pub entity: EntityBundle,
}
