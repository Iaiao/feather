// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Donkey;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct DonkeyBundle {
    pub marker: Donkey,
    #[bundle]
    pub entity: EntityBundle,
}
