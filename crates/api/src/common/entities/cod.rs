// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Cod;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CodBundle {
    pub marker: Cod,
    #[bundle]
    pub entity: EntityBundle,
}
