// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::PolarBear;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PolarBearBundle {
    pub marker: PolarBear,
    #[bundle]
    pub entity: EntityBundle,
}
