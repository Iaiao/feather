use crate::{ChunkPosition, EntityDimension, EntityWorld};
use ahash::AHashSet;
use bevy::ecs::component::Component;
use itertools::Either;

/// The view of a player, representing the set of chunks
/// within their view distance.
#[derive(Clone, Debug, Component)]
pub struct View {
    center: ChunkPosition,
    view_distance: u32,
    world: EntityWorld,
    dimension: EntityDimension,
}

impl View {
    /// Creates a `View` from a center chunk (the position of the player)
    /// and the view distance.
    pub fn new(
        center: ChunkPosition,
        view_distance: u32,
        world: EntityWorld,
        dimension: EntityDimension,
    ) -> Self {
        Self {
            center,
            view_distance,
            world,
            dimension,
        }
    }

    /// Gets the empty view, i.e., the view containing no chunks.
    pub fn empty(world: EntityWorld, dimension: EntityDimension) -> Self {
        Self::new(ChunkPosition::new(0, 0), 0, world, dimension)
    }

    /// Determines whether this is the empty view.
    pub fn is_empty(&self) -> bool {
        self.view_distance == 0
    }

    pub fn center(&self) -> ChunkPosition {
        self.center
    }

    pub fn view_distance(&self) -> u32 {
        self.view_distance
    }

    pub fn set_center(&mut self, center: ChunkPosition) {
        self.center = center;
    }

    pub fn set_view_distance(&mut self, view_distance: u32) {
        self.view_distance = view_distance;
    }

    /// Iterates over chunks visible to the player.
    pub fn iter(&self) -> impl Iterator<Item = ChunkPosition> {
        if self.is_empty() {
            Either::Left(std::iter::empty())
        } else {
            Either::Right(Self::iter_2d(
                self.min_x(),
                self.min_z(),
                self.max_x(),
                self.max_z(),
            ))
        }
    }

    /// Returns the set of chunks that are in `self` but not in `other`.
    pub fn difference(&self, other: &View) -> Vec<ChunkPosition> {
        if self.dimension != other.dimension || self.world != other.world {
            self.iter().collect()
        } else {
            // PERF: consider analytical approach instead of sets
            let self_chunks: AHashSet<_> = self.iter().collect();
            let other_chunks: AHashSet<_> = other.iter().collect();
            self_chunks.difference(&other_chunks).copied().collect()
        }
    }

    /// Determines whether the given chunk is visible.
    pub fn contains(&self, pos: ChunkPosition) -> bool {
        pos.x >= self.min_x()
            && pos.x <= self.max_x()
            && pos.z >= self.min_z()
            && pos.z <= self.max_z()
    }

    fn iter_2d(
        min_x: i32,
        min_z: i32,
        max_x: i32,
        max_z: i32,
    ) -> impl Iterator<Item = ChunkPosition> {
        (min_x..=max_x)
            .flat_map(move |x| (min_z..=max_z).map(move |z| (x, z)))
            .map(|(x, z)| ChunkPosition { x, z })
    }

    /// Returns the minimum X chunk coordinate.
    pub fn min_x(&self) -> i32 {
        // I don't know why but it's loading a 3x3 area with view_distance=2,
        // there should be a better way to fix this
        self.center.x - self.view_distance as i32 - 1
    }

    /// Returns the minimum Z coordinate.
    pub fn min_z(&self) -> i32 {
        self.center.z - self.view_distance as i32 - 1
    }

    /// Returns the maximum X coordinate.
    pub fn max_x(&self) -> i32 {
        self.center.x + self.view_distance as i32 + 1
    }

    /// Returns the maximum Z coordinate.
    pub fn max_z(&self) -> i32 {
        self.center.z + self.view_distance as i32 + 1
    }

    pub fn dimension(&self) -> &EntityDimension {
        &self.dimension
    }

    pub fn world(&self) -> EntityWorld {
        self.world
    }
}