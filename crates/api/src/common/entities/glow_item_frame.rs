use bevy::ecs::bundle::Bundle;

use super::item_frame::ItemFrameFacing;
use crate::components::entity_markers::GlowItemFrame;

use super::EntityBundle;

#[derive(Bundle)]
pub struct GlowItemFrameBundle {
    marker: GlowItemFrame,
    facing: ItemFrameFacing,
    #[bundle]
    entity: EntityBundle,
}
