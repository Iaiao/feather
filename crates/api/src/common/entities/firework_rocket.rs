// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::FireworkRocket;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct FireworkRocketBundle {
    pub marker: FireworkRocket,
    #[bundle]
    pub entity: EntityBundle,
}
