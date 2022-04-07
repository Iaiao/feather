// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Rabbit;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct RabbitBundle {
    pub marker: Rabbit,
    #[bundle]
    pub entity: EntityBundle,
}
