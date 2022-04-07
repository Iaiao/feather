// This file is @generated. Please do not edit.
use super::EntityBundle;
use crate::components::entity_markers::WanderingTrader;
use bevy::ecs::bundle::Bundle;
#[derive(Bundle)]
pub struct WanderingTraderBundle {
    pub marker: WanderingTrader,
    #[bundle]
    pub entity: EntityBundle,
}
