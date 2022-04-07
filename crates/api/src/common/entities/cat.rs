// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Cat;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CatBundle {
    pub marker: Cat,
    #[bundle]
    pub entity: EntityBundle,
}
