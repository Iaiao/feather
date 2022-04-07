// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Egg;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct EggBundle {
    pub marker: Egg,
    #[bundle]
    pub entity: EntityBundle,
}
