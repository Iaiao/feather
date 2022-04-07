// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ElderGuardian;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ElderGuardianBundle {
    pub marker: ElderGuardian,
    #[bundle]
    pub entity: EntityBundle,
}
