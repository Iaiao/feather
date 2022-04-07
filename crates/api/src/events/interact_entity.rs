use crate::bevy::prelude::Entity;
use crate::core::{Hand, InteractionType};
use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractEntityEvent {
    pub target: Entity,
    pub ty: InteractionType,
    pub target_pos: Option<Vec3f>,
    pub hand: Option<Hand>,
    pub sneaking: bool,
}
