// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Squid;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SquidBundle {
    pub marker: Squid,
    #[bundle]
    pub entity: EntityBundle,
}
