use bevy::ecs::bundle::Bundle;

use crate::components::entity_markers::Painting;
use crate::Direction;

use super::EntityBundle;

#[derive(Bundle)]
pub struct PaintingBundle {
    marker: Painting,
    painting_type: PaintingType,
    direction: PaintingDirection,
    #[bundle]
    entity: EntityBundle,
}

// TODO auto-generate painting types
#[derive(Copy, Clone, PartialEq, Eq, Debug, bevy::ecs::component::Component)]
pub enum PaintingType {
    Todo,
}

impl PaintingType {
    pub fn id(&self) -> i32 {
        match *self {
            PaintingType::Todo => unreachable!(),
        }
    }
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
pub struct PaintingDirection(Direction);
