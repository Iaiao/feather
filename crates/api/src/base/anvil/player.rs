use std::collections::HashMap;
use std::num::NonZeroI32;
use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
};

use bevy::prelude::Bundle;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::{ItemKind, ItemStack};

use crate::base::inventory::*;
use crate::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, Instabreak, Invulnerable,
    WalkSpeed,
};

use super::entity::AnimalData;

/// Represents the contents of a player data file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    // Inherit base entity data
    #[serde(flatten)]
    pub animal: AnimalData,

    #[serde(rename = "playerGameType")]
    pub gamemode: i32,
    #[serde(rename = "previousPlayerGameType")]
    pub previous_gamemode: i32,
    #[serde(rename = "Dimension")]
    pub dimension: String,
    #[serde(rename = "Inventory")]
    pub inventory: Vec<InventorySlot>,
    #[serde(rename = "SelectedItemSlot")]
    pub held_item: i32,
    pub abilities: PlayerAbilities,
}

/// Represents player's abilities (flying, invulnerability, speed, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Bundle)]
pub struct PlayerAbilities {
    #[serde(rename = "walkSpeed")]
    pub walk_speed: WalkSpeed,
    #[serde(rename = "flySpeed")]
    pub fly_speed: CreativeFlyingSpeed,
    #[serde(rename = "mayfly")]
    pub may_fly: CanCreativeFly,
    #[serde(rename = "flying")]
    pub is_flying: CreativeFlying,
    #[serde(rename = "mayBuild")]
    pub may_build: CanBuild,
    #[serde(rename = "instabuild")]
    pub instabreak: Instabreak,
    pub invulnerable: Invulnerable,
}

/// Represents a single inventory slot (including position index).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventorySlot {
    #[serde(rename = "Count")]
    pub count: NonZeroI32,
    #[serde(rename = "Slot", default)]
    pub slot: i8,
    #[serde(rename = "id")]
    pub item: String,
    #[serde(default)]
    pub tag: HashMap<String, nbt::Value>,
}

impl InventorySlot {
    /// Converts an [`ItemStack`] and network protocol index into an [`InventorySlot`].
    pub fn from_network_index(network: usize, stack: &ItemStack) -> Option<Self> {
        let slot = if SLOT_HOTBAR_OFFSET <= network && network < SLOT_HOTBAR_OFFSET + HOTBAR_SIZE {
            // Hotbar
            (network - SLOT_HOTBAR_OFFSET) as i8
        } else if network == SLOT_OFFHAND {
            -106
        } else if SLOT_ARMOR_MIN <= network && network <= SLOT_ARMOR_MAX {
            ((SLOT_ARMOR_MAX - network) + 100) as i8
        } else if SLOT_INVENTORY_OFFSET <= network
            && network < SLOT_INVENTORY_OFFSET + INVENTORY_SIZE
        {
            network as i8
        } else {
            return None;
        };

        Some(Self::from_inventory_index(slot, stack))
    }

    /// Converts an [`ItemStack`] and inventory position index into an [`InventorySlot`].
    pub fn from_inventory_index(slot: i8, stack: &ItemStack) -> Self {
        Self {
            count: NonZeroI32::new(stack.count().get() as i32).unwrap(),
            slot,
            item: stack.item().name().to_owned(),
            tag: stack.tag().clone(),
        }
    }

    /// Converts an NBT inventory index to a network protocol index.
    /// Returns None if the index is invalid.
    pub fn convert_index(&self) -> Option<usize> {
        if 0 <= self.slot && self.slot <= 8 {
            // Hotbar
            Some(SLOT_HOTBAR_OFFSET + (self.slot as usize))
        } else if self.slot == -106 {
            // Offhand
            Some(SLOT_OFFHAND as usize)
        } else if 100 <= self.slot && self.slot <= 103 {
            // Equipment
            Some((108 - self.slot) as usize)
        } else if 9 <= self.slot && self.slot <= 35 {
            // Rest of inventory
            Some(self.slot as usize)
        } else {
            // Unknown index
            None
        }
    }

    pub fn into_nbt_value(self) -> nbt::Value {
        let mut compound = HashMap::new();

        compound.insert(
            String::from("Count"),
            nbt::Value::Byte(self.count.get() as i8),
        );
        compound.insert(String::from("id"), nbt::Value::String(self.item));
        compound.insert(String::from("Slot"), nbt::Value::Byte(self.slot));
        compound.insert(String::from("tag"), nbt::Value::Compound(self.tag));

        nbt::Value::Compound(compound)
    }
}

#[derive(Debug)]
pub struct NoSuchItemError;

impl TryFrom<InventorySlot> for ItemStack {
    type Error = NoSuchItemError;

    fn try_from(slot: InventorySlot) -> Result<Self, Self::Error> {
        let mut item = ItemStack::new(
            ItemKind::from_name(slot.item.as_str()).ok_or(NoSuchItemError)?,
            slot.count.get() as u32,
        )
        .unwrap();
        item.merge_tag(slot.tag);
        Ok(item)
    }
}

pub fn load_player_data(world_dir: &Path, uuid: Uuid) -> Result<PlayerData, anyhow::Error> {
    let file_path = file_path(world_dir, uuid);
    let file = File::open(file_path)?;
    let data = nbt::from_gzip_reader(&file)?;
    Ok(data)
}

pub fn save_player_data(
    world_dir: &Path,
    uuid: Uuid,
    data: &PlayerData,
) -> Result<(), anyhow::Error> {
    fs::create_dir_all(world_dir.join("playerdata"))?;
    let file_path = file_path(world_dir, uuid);
    let mut file = File::create(file_path)?;
    nbt::to_gzip_writer(&mut file, data, None).map_err(anyhow::Error::from)
}

fn file_path(world_dir: &Path, uuid: Uuid) -> PathBuf {
    world_dir.join("playerdata").join(format!("{}.dat", uuid))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Cursor;

    use num_traits::ToPrimitive;

    use crate::items::tag_keys;

    use crate::prelude::Gamemode;

    use super::*;

    #[test]
    fn test_deserialize_player() {
        let mut cursor = Cursor::new(include_bytes!("player.dat").to_vec());

        let player: PlayerData = nbt::from_gzip_reader(&mut cursor).unwrap();
        assert_eq!(player.gamemode, Gamemode::Creative.to_i32().unwrap());
        assert_eq!(
            player.previous_gamemode,
            Gamemode::Spectator.to_i32().unwrap()
        );
        assert_eq!(player.inventory[0].item, "minecraft:diamond_shovel");
        assert_eq!(
            player.inventory[0].tag,
            HashMap::from_iter([(tag_keys::DAMAGE.to_string(), nbt::Value::Byte(3))])
        );
    }

    #[test]
    fn test_convert_item() {
        let slot = InventorySlot {
            count: NonZeroI32::new(1).unwrap(),
            slot: 2,
            item: String::from(ItemKind::Feather.name()),
            tag: Default::default(),
        };

        let item_stack: ItemStack = slot.try_into().unwrap();
        assert_eq!(item_stack.item(), ItemKind::Feather);
        assert_eq!(item_stack.count().get(), 1);
    }

    #[test]
    fn test_convert_item_tags() {
        let slot = InventorySlot {
            count: NonZeroI32::new(1).unwrap(),
            slot: 2,
            item: String::from(ItemKind::DiamondAxe.name()),
            tag: HashMap::from_iter([(tag_keys::DAMAGE.to_string(), nbt::Value::Int(42))]),
        };

        let item_stack: ItemStack = slot.try_into().unwrap();
        println!("{:?}", item_stack);
        assert_eq!(item_stack.item(), ItemKind::DiamondAxe);
        assert_eq!(item_stack.count().get(), 1);
        assert_eq!(item_stack.damage_taken(), Some(42));
    }

    #[test]
    fn test_convert_item_unknown_type() {
        let slot = InventorySlot {
            count: NonZeroI32::new(1).unwrap(),
            slot: 2,
            item: String::from("invalid:identifier"),
            tag: Default::default(),
        };

        let item_stack: Result<ItemStack, NoSuchItemError> = slot.try_into();
        assert!(item_stack.is_err());
    }

    #[test]
    fn test_convert_slot_index() {
        let mut map: HashMap<i8, usize> = HashMap::new();

        // Equipment
        map.insert(103, SLOT_ARMOR_HEAD);
        map.insert(102, SLOT_ARMOR_CHEST);
        map.insert(101, SLOT_ARMOR_LEGS);
        map.insert(100, SLOT_ARMOR_FEET);
        map.insert(-106, SLOT_OFFHAND);

        // Hotbar
        for x in 0..9 {
            map.insert(x, SLOT_HOTBAR_OFFSET + (x as usize));
        }

        // Rest of inventory
        for x in 9..36 {
            map.insert(x, x as usize);
        }

        // Check all valid slots
        for (src, expected) in map {
            let slot = InventorySlot {
                slot: src,
                count: NonZeroI32::new(1).unwrap(),
                item: String::from(ItemKind::Stone.name()),
                tag: Default::default(),
            };
            assert_eq!(slot.convert_index().unwrap(), expected);
            assert_eq!(
                InventorySlot::from_network_index(
                    expected,
                    &ItemStack::new(ItemKind::Stone, 1).unwrap()
                ),
                Some(slot),
            );
        }

        // Check that invalid slots error out
        for invalid_slot in [-1, -2, 104].iter() {
            let slot = InventorySlot {
                slot: *invalid_slot as i8,
                count: NonZeroI32::new(1).unwrap(),
                item: String::from("invalid:identifier"),
                tag: Default::default(),
            };
            assert!(slot.convert_index().is_none());
        }
    }
}