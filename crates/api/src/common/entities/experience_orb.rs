use bevy::ecs::bundle::Bundle;

use crate::components::entity_markers::ExperienceOrb;

use super::EntityBundle;

#[derive(Bundle)]
pub struct ExperienceOrbBundle {
    marker: ExperienceOrb,
    amount: ExperienceOrbAmount,
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
    derive_more::DerefMut,
    derive_more::Add,
    derive_more::From,
    bevy::ecs::component::Component,
)]
pub struct ExperienceOrbAmount(u16);
