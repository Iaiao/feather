//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

pub mod chunk;
pub mod entities;
mod generic_bundles;
pub mod interactable;
pub mod positions;
mod region_worker;
pub mod velocity;
pub mod window;
pub mod world;
