use bevy::ecs::bundle::Bundle;

use crate::core::Gamemode;

use crate::base::anvil::player::PlayerAbilities;
use crate::components::entity_markers::Player;
use crate::{
    ChatBox, ClientId, External, HotbarSlot, LivingEntityBundle, PlayerName, PreviousGamemode,
    ProfileProperty, Sneaking, Sprinting, View, Window,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub window: Window,
    pub hotbar_slot: HotbarSlot,
    pub client_id: ClientId,
    pub view: View,
    pub gamemode: Gamemode,
    pub previous_gamemode: PreviousGamemode,
    pub name: PlayerName,
    pub properties: External<Vec<ProfileProperty>>,
    pub chatbox: ChatBox,
    pub sneaking: Sneaking,
    pub sprinting: Sprinting,
    #[bundle]
    pub abilities: PlayerAbilities,
    #[bundle]
    pub living: LivingEntityBundle,
}
