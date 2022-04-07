// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Blaze;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct BlazeBundle {
    pub marker: Blaze,
    #[bundle]
    pub entity: EntityBundle,
}
