// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::ShulkerBullet;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct ShulkerBulletBundle {
    pub marker: ShulkerBullet,
    #[bundle]
    pub entity: EntityBundle,
}
