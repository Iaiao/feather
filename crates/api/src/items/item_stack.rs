use core::fmt::Display;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;

use nbt::Value;
use serde::{Deserialize, Serialize};

use crate::text::Text;

use crate::items::ItemKind;

pub mod tag_keys {
    pub const DAMAGE: &str = "Damage";
    pub const DISPLAY: &str = "display";
    pub const NAME: &str = "Name";
    pub const LORE: &str = "Lore";
}

/// Represents an item stack.
///
/// An item stack includes an item type, an amount and a bunch of properties (enchantments, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemStack {
    /// The item type of this `ItemStack`.
    #[serde(rename = "id")]
    item: ItemKind,

    /// The number of items in the `ItemStack`.
    #[serde(rename = "Count")]
    count: NonZeroU32,

    /// The `ItemStack` metadata, containing data such as damage,
    /// repair cost, enchantments...
    #[serde(default)]
    tag: HashMap<String, nbt::Value>,
}

impl ItemStack {
    /// Creates a new `ItemStack` with the default name (title)
    /// no lore, no damage, no repair cost and no enchantments.
    pub fn new(item: ItemKind, count: u32) -> Result<Self, ItemStackError> {
        let count = NonZeroU32::new(count);
        if count.is_none() {
            return Err(ItemStackError::EmptyStack);
        }
        Ok(Self {
            item,
            count: count.unwrap(),
            tag: Default::default(),
        })
    }

    /// Returns whether the given item stack has
    /// the same type as (but not necessarily the same
    /// amount as) `self`.
    pub fn has_same_type(&self, other: &Self) -> bool {
        self.item == other.item
    }

    /// Returns whether the given `ItemStack` has
    /// the same count as (but not necessarily the same
    /// type as) `self`.
    pub fn has_same_count(&self, other: &Self) -> bool {
        self.count == other.count
    }

    pub fn has_same_tag(&self, other: &Self) -> bool {
        self.tag == other.tag
    }

    /// Returns whether the given `ItemStack` has the same
    /// type and count as (but not necessarily the same tag
    /// as) `self`.
    pub fn has_same_type_and_count(&self, other: &Self) -> bool {
        self.has_same_type(other) && self.has_same_count(other)
    }

    /// Returns whether the given `ItemStack` has the same
    /// type and tag as (but not necessarily the count
    /// as) `self`.
    pub fn is_similar(&self, other: &Self) -> bool {
        self.has_same_type(other) && self.has_same_tag(other)
    }

    /// Returns the item type for this `ItemStack`.
    pub fn item(&self) -> ItemKind {
        self.item
    }

    pub fn tag(&self) -> &HashMap<String, nbt::Value> {
        &self.tag
    }

    /// Returns the number of items in this `ItemStack`.
    pub fn count(&self) -> NonZeroU32 {
        self.count
    }

    /// Adds more items to this `ItemStack`. Returns the new count.
    pub fn add(&mut self, count: u32) -> Result<u32, ItemStackError> {
        let count = self.count.get() + count;
        self.set_count(count)?;
        Ok(count)
    }

    /// Removes some items from this `ItemStack`.
    pub fn remove(&mut self, count: u32) -> Result<u32, ItemStackError> {
        if self.count.get() <= count {
            return Err(if self.count.get() == count {
                ItemStackError::EmptyStack
            } else {
                ItemStackError::NotEnoughAmount
            });
        }
        self.count = NonZeroU32::new(self.count.get() - count).unwrap();
        Ok(self.count.get())
    }

    /// Sets the item type for this `ItemStack`. Returns the new
    /// item type or fails if the current item count exceeds the
    /// new item type stack size.
    pub fn set_item(&mut self, item: ItemKind) -> Result<ItemKind, ItemStackError> {
        if self.count.get() > item.stack_size() {
            return Err(ItemStackError::ExceedsStackSize);
        }
        self.item = item;
        Ok(self.item)
    }

    /// Gets the `ItemStack` and returns it.
    pub fn get_item(&self) -> ItemStack {
        ItemStack {
            count: 1.try_into().unwrap(),
            ..self.clone()
        }
    }

    /// Sets the item type for this `ItemStack`. Does not check if
    /// the new item type stack size will be lower than the current
    /// item count. Returns the new item type.
    pub fn unchecked_set_item(&mut self, item: ItemKind) -> ItemKind {
        self.item = item;
        self.item
    }

    /// Sets the count for this `ItemStack`. Fails if the new
    /// count would exceed the stack size for that item type.
    pub fn set_count(&mut self, count: u32) -> Result<(), ItemStackError> {
        let count = NonZeroU32::new(count).ok_or(ItemStackError::EmptyStack)?;
        if count.get() > self.item.stack_size() {
            return Err(ItemStackError::ExceedsStackSize);
        }
        self.count = count;
        Ok(())
    }

    /// Splits this `ItemStack` in half, returning the
    /// removed half. If the amount is odd, `self`
    /// will be left with the least items. Returns the taken
    /// half.
    pub fn take_half(self) -> (Option<ItemStack>, ItemStack) {
        let half = (self.count.get() + 1) / 2;
        self.take(NonZeroU32::new(half).unwrap())
    }

    /// Splits this `ItemStack` by removing the
    /// specified amount. Returns the taken part.
    pub fn take(mut self, amount: NonZeroU32) -> (Option<ItemStack>, ItemStack) {
        if self.count < amount {
            return (None, self);
        }
        let count_left: u32 = self.count.get() - amount.get();
        let taken = ItemStack {
            count: amount,
            ..self.clone()
        };
        self.count = NonZeroU32::new(count_left).unwrap();
        (Some(self), taken)
    }

    /// Merges another `ItemStack` with this one.
    pub fn merge_with(&mut self, other: Self) -> Result<(), ItemStackError> {
        if !self.is_similar(&other) {
            return Err(ItemStackError::IncompatibleStacks);
        }
        let new_count = (self.count.get() + other.count.get()).min(self.item.stack_size());
        self.count = NonZeroU32::new(new_count).unwrap();
        //other.count = NonZeroU32::new(other.count().get() - amount_added).unwrap();
        Ok(())
    }

    pub fn merge_tag(&mut self, tags: HashMap<String, nbt::Value>) {
        merge_tags(&mut self.tag, tags);
    }

    /// Transfers up to `n` items to `other`.
    pub fn transfer_to(&mut self, n: u32, other: &mut Self) -> Result<(), ItemStackError> {
        if self.count.get() <= n || n == 0 {
            return Err(if self.count.get() == n || n == 0 {
                ItemStackError::EmptyStack
            } else {
                ItemStackError::NotEnoughAmount
            });
        }
        let max_transfer = other.item.stack_size().saturating_sub(other.count.get());
        let transfer = max_transfer.min(self.count.get()).min(n);
        if other.count.get() + transfer > i32::MAX as u32 {
            return Err(ItemStackError::ClientOverflow);
        }

        self.count = NonZeroU32::new(self.count.get() - transfer).unwrap();
        other.count = NonZeroU32::new(other.count.get() + transfer).unwrap();
        Ok(())
    }

    pub fn drain_into_bounded(
        mut self,
        n: u32,
        other: &mut Self,
    ) -> Result<Option<Self>, ItemStackError> {
        if !self.has_same_type(other) {
            return Err(ItemStackError::IncompatibleStacks);
        }

        // Stack size is the same for both self and other because they are the same type.
        let stack_size = self.item.stack_size();
        let space_in_other = stack_size - other.count().get();
        let items_in_self = self.count().get();
        let moving_items = space_in_other.min(n).min(items_in_self);

        other.set_count(moving_items + other.count().get()).unwrap();

        if self.count().get() - moving_items == 0 {
            Ok(None)
        } else {
            self.set_count(moving_items - items_in_self).unwrap();
            Ok(Some(self))
        }
    }

    /// Gets the damage value of this item, fixes the NBT if corrupted (e.g. damage is a string),
    /// inserts a default value if needed.
    fn _damage(&mut self) -> &mut i32 {
        let damage = self
            .tag
            .entry(tag_keys::DAMAGE.to_string())
            .or_insert(nbt::Value::Int(0));
        match damage {
            nbt::Value::Int(i) => i,
            _ => {
                *damage = nbt::Value::Int(0);
                match damage {
                    Value::Int(i) => i,
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Damages the item by the specified amount.
    /// If this function returns `true`, then the item is broken.
    pub fn damage(&mut self, amount: i32) -> bool {
        let damage = self._damage();
        *damage += amount;
        let damage = *damage;
        if let Some(durability) = self.item.max_durability() {
            // This unwrap would only fail if our generated file contains an erroneous
            // default damage value.
            damage >= durability.try_into().unwrap()
        } else {
            false
        }
    }

    /// Returns the amount of damage the items have taken.
    pub fn damage_taken(&self) -> Option<i32> {
        self.tag
            .get(tag_keys::DAMAGE)
            .and_then(|value| match value {
                nbt::Value::Int(i) => Some(*i),
                _ => None,
            })
    }

    /// Returns true is the contents of other could be merged with the contents
    /// of self. This does not look at the item count, just the kind.
    /// Items can be merged when they have the same kind, damage, and enchantment.
    /// If a item has a stacksize of one then it can never be stacked.
    pub fn stackable_types(&self, other: &Self) -> bool {
        self.has_same_type(other)
            && self.has_same_tag(other)
            && self.stack_size() > 1
            && other.stack_size() > 1
    }

    /// How many items could be stacked together
    pub fn stack_size(&self) -> u32 {
        self.item.stack_size()
    }

    /// Returns a nbt representation of this item in format {id="minecraft:item",Count:1b,tag:{...}}
    pub fn into_nbt(self) -> HashMap<String, nbt::Value> {
        let mut compound = HashMap::new();
        compound.insert(
            String::from("Count"),
            nbt::Value::Byte(self.count.get() as i8),
        );
        compound.insert(
            String::from("id"),
            nbt::Value::String(self.item.namespaced_id().to_owned()),
        );
        compound.insert(String::from("tag"), nbt::Value::Compound(self.tag));
        compound
    }
}

/// An error type that may be returned when performing
/// operations over an `ItemStack`.
#[derive(Debug, Clone)]
pub enum ItemStackError {
    ClientOverflow,
    EmptyStack,
    ExceedsStackSize,
    IncompatibleStacks,
    NotEnoughAmount,
}

impl Display for ItemStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ItemStackError {}

pub struct ItemStackBuilder {
    item: ItemKind,
    count: NonZeroU32,
    tag: HashMap<String, nbt::Value>,
}

impl Default for ItemStackBuilder {
    fn default() -> Self {
        Self {
            item: ItemKind::Stone,
            count: 1.try_into().unwrap(),
            tag: Default::default(),
        }
    }
}

impl ItemStackBuilder {
    pub fn new(item: ItemKind) -> Self {
        Self {
            item,
            count: 1.try_into().unwrap(),
            tag: Default::default(),
        }
    }

    pub fn item(self, item: ItemKind) -> Self {
        Self { item, ..self }
    }

    // panics if the count is zero
    pub fn count(self, count: u32) -> Self {
        Self {
            count: count.try_into().unwrap(),
            ..self
        }
    }

    pub fn title(mut self, title: impl Into<Text>) -> Self {
        let display = self.get_or_create_tag(tag_keys::DISPLAY);
        display.insert(
            tag_keys::NAME.to_string(),
            nbt::Value::String(title.into().to_json()),
        );
        self
    }

    pub fn lore(mut self, lore: Vec<impl Into<Text>>) -> Self {
        let display = self.get_or_create_tag(tag_keys::DISPLAY);
        display.insert(
            tag_keys::LORE.to_string(),
            nbt::Value::List(
                lore.into_iter()
                    .map(|line| nbt::Value::String(line.into().to_json()))
                    .collect(),
            ),
        );
        self
    }

    pub fn damage(mut self, damage: i32) -> Self {
        self.tag
            .insert(tag_keys::DAMAGE.to_string(), nbt::Value::Int(damage));
        self
    }

    pub fn clone_tag_from(mut self, other: &Self) -> Self {
        self.tag = other.tag.clone();
        self
    }

    pub fn build(self) -> ItemStack {
        self.into()
    }

    fn get_or_create_tag(&mut self, s: &str) -> &mut HashMap<String, nbt::Value> {
        if !self.tag.contains_key(s) {
            self.tag
                .insert(s.to_owned(), nbt::Value::Compound(Default::default()));
        }
        match self.tag.get_mut(s).unwrap() {
            nbt::Value::Compound(map) => map,
            // this fn is private and the tag can't be modified from outside of this builder
            _ => panic!("The tag is not a compound tag"),
        }
    }
}

impl From<ItemStackBuilder> for ItemStack {
    fn from(it: ItemStackBuilder) -> Self {
        Self {
            item: it.item,
            count: it.count,
            tag: it.tag,
        }
    }
}

pub fn merge_tags(into: &mut HashMap<String, nbt::Value>, from: HashMap<String, nbt::Value>) {
    for (key, tag) in from {
        if let Some(entry) = into.get_mut(&key) {
            match (entry, tag) {
                (nbt::Value::Compound(into), nbt::Value::Compound(from)) => merge_tags(into, from),
                (into, from) => *into = from,
            }
        } else {
            into.insert(key, tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::num::NonZeroU32;

    use crate::text::{TextComponent, TextComponentBuilder};

    use crate::prelude::tag_keys::*;
    use crate::{ItemKind, ItemStack, ItemStackBuilder};

    #[test]
    fn test_builder() {
        let stone = ItemStackBuilder::new(ItemKind::Stone)
            .count(5)
            .title("A stone block.")
            .build();
        assert_eq!(
            stone,
            ItemStack {
                item: ItemKind::Stone,
                count: NonZeroU32::new(5).unwrap(),
                tag: HashMap::from_iter([(
                    DISPLAY.to_string(),
                    nbt::Value::Compound(HashMap::from_iter([(
                        NAME.to_string(),
                        nbt::Value::String("\"A stone block.\"".to_string())
                    )]))
                )])
            }
        );

        let sword = ItemStackBuilder::new(ItemKind::NetheriteSword)
            .damage(100)
            .title(TextComponent::from("Something").red())
            .lore(vec!["First line", "Second line"])
            .build();
        assert_eq!(
            sword,
            ItemStack {
                item: ItemKind::NetheriteSword,
                count: NonZeroU32::new(1).unwrap(),
                tag: HashMap::from_iter([
                    (DAMAGE.to_string(), nbt::Value::Int(100),),
                    (
                        DISPLAY.to_string(),
                        nbt::Value::Compound(HashMap::from_iter([
                            (
                                NAME.to_string(),
                                nbt::Value::String(
                                    "{\"text\":\"Something\",\"color\":\"red\"}".to_string()
                                )
                            ),
                            (
                                LORE.to_string(),
                                nbt::Value::List(vec![
                                    nbt::Value::String("\"First line\"".to_string()),
                                    nbt::Value::String("\"Second line\"".to_string()),
                                ])
                            )
                        ]))
                    )
                ])
            }
        );
    }

    #[test]
    fn test_merge() {
        let mut item = ItemStackBuilder::new(ItemKind::Tnt).build();
        // Add {Tag1={Tag11=1}}
        item.merge_tag(HashMap::from_iter([(
            "Tag1".to_string(),
            nbt::Value::Compound(HashMap::from_iter([(
                "Tag11".to_string(),
                nbt::Value::Int(1),
            )])),
        )]));
        assert_eq!(
            item.tag,
            HashMap::from_iter([(
                "Tag1".to_string(),
                nbt::Value::Compound(HashMap::from_iter([(
                    "Tag11".to_string(),
                    nbt::Value::Int(1)
                )])),
            )])
        );
        // Add {Tag1={Tag12=2},Tag2={Tag21=4}}
        item.merge_tag(HashMap::from_iter([
            (
                "Tag1".to_string(),
                nbt::Value::Compound(HashMap::from_iter([(
                    "Tag12".to_string(),
                    nbt::Value::Int(2),
                )])),
            ),
            (
                "Tag2".to_string(),
                nbt::Value::Compound(HashMap::from_iter([(
                    "Tag21".to_string(),
                    nbt::Value::Int(4),
                )])),
            ),
        ]));
        assert_eq!(
            item.tag,
            HashMap::from_iter([
                (
                    "Tag1".to_string(),
                    nbt::Value::Compound(HashMap::from_iter([
                        ("Tag11".to_string(), nbt::Value::Int(1)),
                        ("Tag12".to_string(), nbt::Value::Int(2))
                    ])),
                ),
                (
                    "Tag2".to_string(),
                    nbt::Value::Compound(HashMap::from_iter([(
                        "Tag21".to_string(),
                        nbt::Value::Int(4)
                    )])),
                )
            ])
        );
    }
}
