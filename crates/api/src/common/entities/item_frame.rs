use super::EntityBundle;
use crate::blocks::FacingCubic;
use crate::components::entity_markers::ItemFrame;
use bevy::ecs::bundle::Bundle;

#[derive(Bundle)]
pub struct ItemFrameBundle {
    marker: ItemFrame,
    facing: ItemFrameFacing,
    #[bundle]
    entity: EntityBundle,
}

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Deref,
    derive_more::From,
    bevy::ecs::component::Component,
)]
pub struct ItemFrameFacing(pub FacingCubic);
