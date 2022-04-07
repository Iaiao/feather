// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Slime;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct SlimeBundle {
    pub marker: Slime,
    #[bundle]
    pub entity: EntityBundle,
}
