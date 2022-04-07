//! Components not associated with a specific type of entity.
//!
//! See the [entities module](crate::entities) for entity-specific
//! components.

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use bevy::ecs::component::{ComponentStorage, SparseStorage, TableStorage};
use bevy::prelude::*;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::Gamemode;
use crate::text::Text;
use crate::text::Title;
use crate::ChunkPosition;
pub use entity_markers::*;

use crate::prelude::Position;

pub mod entity_markers;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct External<T, S: ComponentStorage = TableStorage>(pub T, PhantomData<S>);

// For some reason External<Uuid> wasn't Copy + Clone with derive
impl<T: Clone, S: ComponentStorage> Clone for External<T, S> {
    fn clone(&self) -> Self {
        External(self.0.clone(), PhantomData)
    }
}

impl<T: Copy, S: ComponentStorage> Copy for External<T, S> {}

impl<T: Send + Sync + 'static, S: ComponentStorage + Send + Sync + 'static> Component
    for External<T, S>
{
    type Storage = S;
}

impl<T> External<T> {
    pub fn new(t: T) -> External<T, TableStorage> {
        External(t, PhantomData::<TableStorage>)
    }
    pub fn new_sparse(t: T) -> External<T, SparseStorage> {
        External(t, PhantomData::<SparseStorage>)
    }
}

impl<T> Deref for External<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for External<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(
    Clone, PartialEq, Eq, Hash, Debug, derive_more::Deref, derive_more::DerefMut, Component,
)]
pub struct EntityDimension(pub DimensionId);

#[derive(
    Clone, PartialEq, Eq, Hash, Debug, derive_more::Deref, derive_more::DerefMut, Component,
)]
pub struct DimensionId(pub String);

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Into,
    Component,
)]
pub struct EntityWorld(pub WorldId);

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::Into,
    Component,
)]
pub struct WorldId(pub Entity);

/// Whether an entity is touching the ground.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct OnGround(pub bool);

/// A player's username.
///
/// This component is immutable. Do not
/// attempt to change it.
///
/// Non-player entities cannot have this component. See [`CustomName`]
/// if you need to name an entity.
#[derive(
    Clone, Debug, Serialize, Deserialize, derive_more::Deref, derive_more::Constructor, Component,
)]
pub struct PlayerName(String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An entity's custom name.
///
/// Adding this component to an entity
/// will give it a custom name, visible on the client.
///
/// Giving a player a custom name has no effect.
#[derive(
    Clone, Debug, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut, Component,
)]
pub struct CustomName(pub String);

/// A player's walk speed
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct WalkSpeed(pub f32);

impl Default for WalkSpeed {
    fn default() -> Self {
        WalkSpeed(0.1)
    }
}

/// A player's fly speed
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct CreativeFlyingSpeed(pub f32);

impl Default for CreativeFlyingSpeed {
    fn default() -> Self {
        CreativeFlyingSpeed(0.05)
    }
}

/// Whether a player can fly like in creative mode
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct CanCreativeFly(pub bool);

/// Whether a player is flying (like in creative mode, so it does not reflect if the player is flying by other means)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct CreativeFlying(pub bool);

/// Whether a player can place and destroy blocks
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct CanBuild(pub bool);

/// Whether a player breaks blocks instantly (like in creative mode)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct Instabreak(pub bool);

/// Whether a player is immune to damage
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct Invulnerable(pub bool);

/// Whether an entity is sneaking, like in pressing shift.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct Sneaking(pub bool);

/// A player's previous gamemode
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct PreviousGamemode(pub Option<Gamemode>);

impl PreviousGamemode {
    /// Gets a previous gamemode from its ID.
    pub fn from_id(id: i8) -> Self {
        PreviousGamemode(match id {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None,
        })
    }

    /// Gets this gamemode's id
    pub fn id(&self) -> i8 {
        match self.0 {
            Some(Gamemode::Survival) => 0,
            Some(Gamemode::Creative) => 1,
            Some(Gamemode::Adventure) => 2,
            Some(Gamemode::Spectator) => 3,
            None => -1,
        }
    }
}

/// Represents an entity's health
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct Health(pub f32);

/// A component on players that tracks if they are sprinting or not.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
    Component,
)]
pub struct Sprinting(pub bool);
impl Sprinting {
    pub fn new(value: bool) -> Self {
        Sprinting(value)
    }
}

/// The hotbar slot a player's cursor is currently on
#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Component, derive_more::Deref, derive_more::DerefMut,
)]
pub struct HotbarSlot(usize);

impl HotbarSlot {
    pub fn new(id: usize) -> anyhow::Result<Self> {
        if id > 8 {
            anyhow::bail!("invalid hotbar slot id");
        }
        Ok(Self(id))
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn set(&mut self, id: usize) -> anyhow::Result<()> {
        if id > 8 {
            anyhow::bail!("invalid hotbar slot id");
        }

        self.0 = id;
        Ok(())
    }
}

/// An entity's "mailbox" for receiving chat messages.
///
/// Internally stores a list of [`ChatMessage`]s.
/// (`feather-server` flushes mailboxes by sending chat packets.)
#[derive(Debug, Component)]
pub struct ChatBox {
    messages: Vec<(Option<Uuid>, ChatMessage)>,
    titles: Vec<Title>,
    preference: ChatPreference,
}

impl ChatBox {
    pub fn new(preference: ChatPreference) -> Self {
        Self {
            messages: Vec::new(),
            titles: Vec::new(),
            preference,
        }
    }

    pub fn set_preference(&mut self, preference: ChatPreference) {
        self.preference = preference;
    }

    pub fn send(&mut self, sender: Option<Uuid>, message: ChatMessage) {
        self.messages.push((sender, message));
    }

    pub fn send_chat(&mut self, sender: Option<Uuid>, message: impl Into<Text>) {
        self.send(
            sender,
            ChatMessage::new(ChatKind::PlayerChat, message.into()),
        );
    }

    pub fn send_system(&mut self, message: impl Into<Text>) {
        self.send(None, ChatMessage::new(ChatKind::System, message.into()));
    }

    pub fn send_above_hotbar(&mut self, message: impl Into<Text>) {
        self.send(
            None,
            ChatMessage::new(ChatKind::AboveHotbar, message.into()),
        );
    }

    /// Adds the [`Title`] to the title queue.
    pub fn send_title(&mut self, title: Title) {
        self.titles.push(title);
    }

    /// Drains titles in the mailbox
    pub fn drain_titles(&mut self) -> impl Iterator<Item = Title> + '_ {
        self.titles.drain(..)
    }

    /// Drains messages in the mailbox.
    pub fn drain(&mut self) -> impl Iterator<Item = (Option<Uuid>, ChatMessage)> + '_ {
        let preference = self.preference;
        self.messages
            .drain(..)
            .filter(move |(_, msg)| msg.kind.should_send(preference))
    }
}

/// Represents a chat message.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    kind: ChatKind,
    message: Text,
}

impl ChatMessage {
    pub fn new(kind: ChatKind, message: Text) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> ChatKind {
        self.kind
    }

    pub fn text(&self) -> &Text {
        &self.message
    }
}

/// Kind of chat message. The client determines whether
/// to display a message based on this kind.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChatKind {
    /// A player chat message or similar.
    PlayerChat,
    /// The output of a command or other messages
    /// not originating from players.
    System,
    /// A message displayed above the hotbar.
    AboveHotbar,
}

impl ChatKind {
    pub fn should_send(self, preference: ChatPreference) -> bool {
        match self {
            ChatKind::PlayerChat => preference == ChatPreference::All,
            ChatKind::System => preference >= ChatPreference::System,
            ChatKind::AboveHotbar => true,
        }
    }
}

/// A player's chat preference.
/// Determines which [`ChatKind`]s will
/// be sent to this player.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Component)]
pub enum ChatPreference {
    /// Receive only game info messages.
    GameInfoOnly,
    /// Receive only messages from commands and game info messages.
    System,
    /// Receive all messages.
    All,
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive, Component,
)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A profile property, which stores metadata
/// for some player's account. This is usually
/// used to store skin data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

/// Stores the [`Position`] of an entity on
/// the previous tick. Used to determine
/// when to send movement updates.
#[derive(Copy, Clone, Debug, Component)]
pub struct PreviousPosition(pub Position);
/// Stores the [`OnGround`] status of an entity on
/// the previous tick. Used to determine
/// what movement packet to send.
#[derive(Copy, Clone, Debug, Component)]
pub struct PreviousOnGround(pub OnGround);

#[derive(Eq, PartialEq, Clone, Debug, Hash, Component)]
pub struct DimensionChunkPosition {
    world: WorldId,
    dimension: DimensionId,
    position: ChunkPosition,
}

impl DimensionChunkPosition {
    pub fn new(world: WorldId, dimension: DimensionId, position: ChunkPosition) -> Self {
        DimensionChunkPosition {
            world,
            dimension,
            position,
        }
    }

    pub fn world(&self) -> WorldId {
        self.world
    }

    pub fn dimension(&self) -> &DimensionId {
        &self.dimension
    }

    pub fn position(&self) -> ChunkPosition {
        self.position
    }
}
