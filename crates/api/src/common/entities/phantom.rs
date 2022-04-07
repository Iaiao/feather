// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::Phantom;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct PhantomBundle {
    pub marker: Phantom,
    #[bundle]
    pub entity: EntityBundle,
}
