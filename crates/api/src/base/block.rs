use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::{BlockPosition, ChunkPosition, Position};
use std::convert::TryFrom;

/// Validated position of a block.
///
/// This structure is immutable.
/// All operations that change a [`ValidBlockPosition`] must be done by
/// turning it into a [`BlockPosition`], performing said operations,
/// then using [`ValidBlockPosition`]'s [`TryFrom`] impl to get a [`ValidBlockPosition`].
///
/// The definition of a valid block position is defined by [`BlockPosition::valid`].
///
/// # Examples
///
/// Converting a [`BlockPosition`] to a [`ValidBlockPosition`], unwrapping any errors that
/// occur.
/// ```
/// # use feather_api::prelude::BlockPosition;
/// # use feather_api::prelude::ValidBlockPosition;
/// # use std::convert::TryInto;
/// // Create an unvalidated block position
/// let block_position = BlockPosition::new(727, 32, 727);
///
/// // Validate the block position and unwrap any errors
/// let valid_block_position: ValidBlockPosition = block_position.try_into().unwrap();
/// ```
///
/// Performing operations on a [`ValidBlockPosition`], then re-validating it.
/// ```
/// # use feather_api::prelude::BlockPosition;
/// # use feather_api::prelude::ValidBlockPosition;
/// # use std::convert::TryInto;
/// # let mut valid_block_position: ValidBlockPosition = BlockPosition::new(727, 32, 727).try_into().unwrap();
/// // Convert the ValidBlockPosition into an unvalidated one to perform math
/// let mut block_position: BlockPosition = valid_block_position.into();
///
/// block_position.x = 821;
/// block_position.z += 32;
///
/// assert!(block_position.valid());
///
/// valid_block_position = block_position.try_into().unwrap();
/// ```
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[repr(C)]
pub struct NetworkBlockPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl NetworkBlockPosition {
    pub const fn new(x: i32, y: i32, z: i32) -> Result<Self, BlockPositionValidationError> {
        let value = BlockPosition::new(x, y, z);
        if value.is_network_valid() {
            Ok(NetworkBlockPosition {
                x: value.x,
                y: value.y,
                z: value.z,
            })
        } else {
            Err(BlockPositionValidationError::OutOfRange(value))
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn chunk(self) -> ChunkPosition {
        self.into()
    }

    pub fn position(self) -> Position {
        self.into()
    }
}

impl TryFrom<BlockPosition> for NetworkBlockPosition {
    type Error = <Self as TryFrom<&'static BlockPosition>>::Error;

    fn try_from(value: BlockPosition) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<'a> TryFrom<&'a BlockPosition> for NetworkBlockPosition {
    type Error = BlockPositionValidationError;

    fn try_from(value: &'a BlockPosition) -> Result<Self, Self::Error> {
        if value.is_network_valid() {
            Ok(NetworkBlockPosition {
                x: value.x,
                y: value.y,
                z: value.z,
            })
        } else {
            Err(BlockPositionValidationError::OutOfRange(*value))
        }
    }
}

impl From<NetworkBlockPosition> for BlockPosition {
    fn from(position: NetworkBlockPosition) -> Self {
        BlockPosition {
            x: position.x,
            y: position.y,
            z: position.z,
        }
    }
}

impl From<NetworkBlockPosition> for ChunkPosition {
    fn from(position: NetworkBlockPosition) -> Self {
        let position: BlockPosition = position.into();
        position.into()
    }
}

impl From<NetworkBlockPosition> for Position {
    fn from(position: NetworkBlockPosition) -> Self {
        let position: BlockPosition = position.into();
        position.into()
    }
}

#[derive(Error, Debug)]
pub enum BlockPositionValidationError {
    #[error("coordinate {0:?} out of range")]
    OutOfRange(BlockPosition),
}

#[cfg(test)]
mod tests {

    use crate::base::block::NetworkBlockPosition;
    use std::convert::TryInto;

    use crate::prelude::*;

    #[test]
    #[should_panic]
    fn check_out_of_bounds_up() {
        let block_position = BlockPosition::new(0, 39483298, 0);

        <BlockPosition as TryInto<NetworkBlockPosition>>::try_into(block_position).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_out_of_bounds_down() {
        let block_position = BlockPosition::new(0, -39483298, 0);

        <BlockPosition as TryInto<NetworkBlockPosition>>::try_into(block_position).unwrap();
    }
}
