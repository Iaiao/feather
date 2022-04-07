// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Turtle;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct TurtleBundle {
    pub marker: Turtle,
    #[bundle]
    pub entity: EntityBundle,
}
