// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Chicken;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ChickenBundle {
    pub marker: Chicken,
    #[bundle]
    pub entity: EntityBundle,
}
