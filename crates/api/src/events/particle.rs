use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct ParticleEvent {
    pub particle: Particle,
    pub long_distance: bool,
    pub position: Position,
    pub world: WorldId,
    pub dimension: DimensionId,
}
