use bevy::ecs::query::QueryEntityError;

use crate::prelude::*;

pub fn set_block_from_query(
    mut query: Query<&mut Dimensions>,
    dimension: &DimensionId,
    world: WorldId,
    pos: BlockPosition,
    id: BlockId,
    event_writer: &mut EventWriter<BlockChangeEvent>,
) -> Result<(), SetBlockError> {
    let mut dimensions = query.get_mut(*world).map_err(|err| match err {
        QueryEntityError::QueryDoesNotMatch => SetBlockError::InvalidWorldEntity,
        QueryEntityError::NoSuchEntity => SetBlockError::NoSuchWorld,
    })?;
    set_block_in(&mut *dimensions, dimension, world, pos, id, event_writer)
}

pub fn set_block_with_world_name(
    mut query: Query<(Entity, &WorldName, &mut Dimensions)>,
    dimension: &DimensionId,
    world: impl AsRef<String>,
    pos: BlockPosition,
    id: BlockId,
    event_writer: &mut EventWriter<BlockChangeEvent>,
) -> Result<(), SetBlockError> {
    let (world, _, mut dimensions) = query
        .iter_mut()
        .find(|(_, name, _)| ***name == *world.as_ref())
        .ok_or(SetBlockError::NoSuchWorld)?;
    set_block_in(
        &mut *dimensions,
        dimension,
        WorldId(world),
        pos,
        id,
        event_writer,
    )
}

pub fn set_block_in(
    dimensions: &mut Dimensions,
    dimension: &DimensionId,
    world: WorldId,
    pos: BlockPosition,
    id: BlockId,
    event_writer: &mut EventWriter<BlockChangeEvent>,
) -> Result<(), SetBlockError> {
    let dimension = dimensions
        .get_mut(dimension)
        .ok_or(SetBlockError::NoSuchDimension)?;
    if set_block_at(dimension, world, pos, id, event_writer) {
        Ok(())
    } else {
        Err(SetBlockError::PositionNotLoaded)
    }
}

pub fn set_block_at(
    dimension: &mut Dimension,
    world: WorldId,
    pos: BlockPosition,
    id: BlockId,
    event_writer: &mut EventWriter<BlockChangeEvent>,
) -> bool {
    event_writer.send(BlockChangeEvent::single(pos, world.into(), dimension.id()));
    dimension.set_block_at(pos, id)
}

#[derive(Debug, thiserror::Error)]
pub enum SetBlockError {
    #[error("World with this ID doesn't exist")]
    NoSuchWorld,
    #[error("This entity ID doesn't belong to a world")]
    InvalidWorldEntity,
    #[error("Dimension with this ID doesn't exist")]
    NoSuchDimension,
    #[error("Position is not loaded. Loading positions from plugin API is a WIP")]
    PositionNotLoaded,
}
