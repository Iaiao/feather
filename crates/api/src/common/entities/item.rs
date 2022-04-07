// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Item;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ItemBundle {
    pub marker: Item,
    #[bundle]
    pub entity: EntityBundle,
}
