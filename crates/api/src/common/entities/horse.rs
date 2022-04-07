// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Horse;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct HorseBundle {
    pub marker: Horse,
    #[bundle]
    pub entity: EntityBundle,
}
