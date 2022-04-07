//! Foundational types and constants for Minecraft.

pub mod biomes;
pub mod block;
mod consts;
mod entity;
mod gamemode;
mod gamerules;
mod interaction;
mod player;

pub use consts::*;
pub use entity::EntityKind;
pub use gamemode::Gamemode;
pub use gamerules::GameRules;
pub use interaction::InteractionType;
pub use player::Hand;
