// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Cow;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct CowBundle {
    pub marker: Cow,
    #[bundle]
    pub entity: EntityBundle,
}
