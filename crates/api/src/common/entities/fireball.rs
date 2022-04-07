// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Fireball;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct FireballBundle {
    pub marker: Fireball,
    #[bundle]
    pub entity: EntityBundle,
}
