// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::GlowSquid;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct GlowSquidBundle {
    pub marker: GlowSquid,
    #[bundle]
    pub entity: EntityBundle,
}
