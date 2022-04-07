// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::IronGolem;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct IronGolemBundle {
    pub marker: IronGolem,
    #[bundle]
    pub entity: EntityBundle,
}
