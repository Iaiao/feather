use std::mem;

use anyhow::bail;
use bevy::prelude::Component;
use parking_lot::MutexGuard;

pub use crate::inventory::Window as BackingWindow;
use crate::inventory::{Area, WindowError};
use crate::items::InventorySlot::{self, Empty};
use crate::items::ItemKind;

use crate::prelude::Inventory;

/// A player's window. Wraps one or more inventories and handles
/// conversion between protocol and slot indices.
///
/// Also provides high-level methods to interact with the inventory,
/// like [`Window::right_click`], [`Window::shift_click`], etc.
#[derive(Debug, Component)]
pub struct Window {
    /// The backing window (contains the `Inventory`s)
    inner: BackingWindow,
    /// The item currently held by the player's cursor.
    cursor_item: InventorySlot,
    /// Current painting state (mouse drag)
    paint_state: Option<PaintState>,
}

impl Window {
    /// Creates a window from the backing window representation.
    pub fn new(inner: BackingWindow) -> Self {
        Self {
            inner,
            cursor_item: Empty,
            paint_state: None,
        }
    }

    /// Left-click a slot in the window.
    pub fn left_click(&mut self, slot: usize) -> Result<(), WindowError> {
        println!("Left click");
        let slot = &mut *self.inner.item(slot)?;
        let cursor_slot = &mut self.cursor_item;

        // Cases:
        // * Either the cursor slot or the clicked slot is empty; swap the two.
        // * Both slots are present but are of different types; swap the two.
        // * Both slots are present and have the same type; merge the two.

        if slot.is_filled() && cursor_slot.is_filled() && cursor_slot.is_mergable(slot) {
            slot.merge(cursor_slot);
        } else {
            mem::swap(cursor_slot, slot);
        }

        Ok(())
    }

    /// Right-clicks a slot in the window.
    pub fn right_click(&mut self, slot_index: usize) -> Result<(), WindowError> {
        let slot = &mut *self.inner.item(slot_index)?;
        let cursor_slot = &mut self.cursor_item;

        // Cases:
        // * Cursor slot is present and clicked slot has the same item type; drop one item in the clicked slot.
        // * Clicked slot is present but cursor slot is not; move half the items into the cursor slot.
        // * Both slots are present but differ in type; swap the two.

        match (slot.is_filled(), cursor_slot.is_filled()) {
            (true, true) => {
                if slot.is_mergable(cursor_slot) {
                    cursor_slot.transfer_to(1, slot);
                } else {
                    mem::swap(slot, cursor_slot);
                }
            }
            (true, false) => {
                *cursor_slot = slot.take_half();
            }
            (false, true) => {
                *slot = cursor_slot.try_take(1);
            }
            (false, false) => {}
        }

        Ok(())
    }

    /// Shift-clicks the given slot. (Either right or left click.)
    pub fn shift_click(&mut self, slot: usize) -> Result<(), WindowError> {
        // If we are shift clicking on a empty slot, then nothing happens.
        {
            let slot_inventory = &mut *self.inner.item(slot)?;
            if slot_inventory.is_empty() {
                // Shift clicking on a empty inventory slot does nothing.
                return Ok(());
            }
        }

        match &self.inner {
            BackingWindow::Player { player: _ } => self.shift_click_in_player_window(slot),

            BackingWindow::Generic9X1 { .. }
            | BackingWindow::Generic9X2 { .. }
            | BackingWindow::Generic9X3 { .. }
            | BackingWindow::Generic9X4 { .. }
            | BackingWindow::Generic9X5 { .. }
            | BackingWindow::Generic3X3 { .. }
            | BackingWindow::Generic9X6 { .. } => self.shift_click_in_generic_window(slot),

            BackingWindow::Crafting { .. } => self.shift_click_in_crafting_window(slot),
            BackingWindow::Furnace { .. } => self.shift_click_in_furnace(slot),

            BackingWindow::BlastFurnace { .. } => self.shift_click_in_blast_furnace(slot),

            BackingWindow::Smoker { .. } => self.shift_click_in_smoker(slot),

            BackingWindow::Enchantment { .. } => self.shift_click_in_enchantment(slot),

            BackingWindow::BrewingStand { .. } => self.shift_click_in_brewing_window(slot),

            BackingWindow::Beacon { .. } => self.shift_click_in_beacon(slot),

            BackingWindow::Anvil { .. } => self.shift_click_in_anvil(slot),
            BackingWindow::Hopper { .. } => self.shift_click_in_hopper(slot),
            BackingWindow::ShulkerBox { .. } => self.shift_click_in_shulker_box(slot),

            BackingWindow::Cartography { .. } => self.shift_click_in_cartography_window(slot),
            BackingWindow::Grindstone { .. } => self.shift_click_in_grindstone(slot),
            BackingWindow::Lectern { .. } => self.shift_click_in_lectern(slot),
            BackingWindow::Loom { loom: _, player: _ } => self.shift_click_in_loom(slot),
            BackingWindow::Stonecutter { .. } => self.shift_click_in_stonecutter(slot),
        }
    }

    pub fn inventory(&self) -> &Inventory {
        match &self.inner {
            BackingWindow::Player { player } => player,
            BackingWindow::Generic9X1 { player, .. } => player,
            BackingWindow::Generic9X2 { player, .. } => player,
            BackingWindow::Generic9X3 { player, .. } => player,
            BackingWindow::Generic9X4 { player, .. } => player,
            BackingWindow::Generic9X5 { player, .. } => player,
            BackingWindow::Generic9X6 { player, .. } => player,
            BackingWindow::Generic3X3 { player, .. } => player,
            BackingWindow::Crafting { player, .. } => player,
            BackingWindow::Furnace { player, .. } => player,
            BackingWindow::BlastFurnace { player, .. } => player,
            BackingWindow::Smoker { player, .. } => player,
            BackingWindow::Enchantment { player, .. } => player,
            BackingWindow::BrewingStand { player, .. } => player,
            BackingWindow::Beacon { player, .. } => player,
            BackingWindow::Anvil { player, .. } => player,
            BackingWindow::Hopper { player, .. } => player,
            BackingWindow::ShulkerBox { player, .. } => player,
            BackingWindow::Cartography { player, .. } => player,
            BackingWindow::Grindstone { player, .. } => player,
            BackingWindow::Lectern { player, .. } => player,
            BackingWindow::Loom { player, .. } => player,
            BackingWindow::Stonecutter { player, .. } => player,
        }
    }

    fn shift_click_in_player_window(&mut self, slot: usize) -> Result<(), WindowError> {
        let slot_item = &mut *self.inner.item(slot)?;

        let (inventory, slot_area, _) = self.inner.index_to_slot(slot).unwrap();
        let areas_to_try = [
            Area::Helmet,
            Area::Chestplate,
            Area::Leggings,
            Area::Boots,
            Area::CraftingInput,
            Area::Hotbar,
            Area::Storage,
        ];

        for &area in &areas_to_try {
            if area == slot_area || !will_accept(area, slot_item) {
                continue;
            }

            // Find slot with same type first
            let mut i = 0;
            while let Some(mut stack) = inventory.item(area, i) {
                if slot_item.is_mergable(&stack) && stack.is_filled() {
                    stack.merge(slot_item);
                }
                i += 1;
            }

            if slot_item.is_empty() {
                return Ok(());
            }
        }

        if slot_item.is_filled() {
            for &area in &areas_to_try {
                if area == slot_area || !will_accept(area, slot_item) {
                    continue;
                }

                // If we still haven't moved all the items, transfer to any empty space
                let mut i = 0;
                while let Some(mut stack) = inventory.item(area, i) {
                    if stack.is_empty() {
                        stack.merge(slot_item);
                    }
                    i += 1;
                }

                if slot_item.is_empty() {
                    break;
                }
            }
        }

        Ok(())
    }

    fn shift_click_in_generic_window(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_crafting_window(&mut self, _slot: usize) -> Result<(), WindowError> {
        // TODO: If you shift click an item in the crafting table, then you craft
        // as many as possible. So the items are crafted and put in Area::CraftingOutput
        // We don't currently have a working crafting system, and once we have we probably
        // need to change the function signature to get acsess to the crafting system.
        todo!()
    }

    fn shift_click_in_furnace(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_blast_furnace(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_smoker(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_enchantment(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_brewing_window(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_beacon(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_anvil(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_hopper(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_shulker_box(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    fn shift_click_in_cartography_window(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }
    fn shift_click_in_grindstone(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }
    fn shift_click_in_lectern(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }
    fn shift_click_in_loom(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }
    fn shift_click_in_stonecutter(&mut self, _slot: usize) -> Result<(), WindowError> {
        todo!()
    }

    /// Starts a left mouse paint operation.
    pub fn begin_left_mouse_paint(&mut self) {
        self.paint_state = Some(PaintState::new(Mouse::Left));
    }

    /// Starts a right mouse paint operation.
    pub fn begin_right_mouse_paint(&mut self) {
        self.paint_state = Some(PaintState::new(Mouse::Right));
    }

    /// Starts a middle mouse paint operation.
    pub fn begin_middle_mouse_paint(&mut self) {
        self.paint_state = Some(PaintState::new(Mouse::Middle));
    }

    /// Adds a slot to the current paint operation.
    pub fn add_paint_slot(&mut self, slot: usize) -> anyhow::Result<()> {
        if let Some(state) = &mut self.paint_state {
            state.add_slot(slot)?;
            Ok(())
        } else {
            bail!("no paint operation was active")
        }
    }

    /// Completes and executes the current paint operation.
    pub fn end_paint(&mut self) -> anyhow::Result<()> {
        if let Some(state) = self.paint_state.take() {
            state.finish(self)?;
            Ok(())
        } else {
            bail!("no paint operation was active")
        }
    }

    /// Gets the item currently held in the cursor.
    pub fn cursor_item(&self) -> &InventorySlot {
        &self.cursor_item
    }

    /// Sets the item currently held in the cursor.
    pub fn set_cursor_item(&mut self, item: InventorySlot) {
        self.cursor_item = item;
    }

    pub fn item(&self, index: usize) -> Result<MutexGuard<InventorySlot>, WindowError> {
        self.inner.item(index)
    }

    /// Sets an [`InventorySlot`] at the index.
    /// # Error
    /// Returns an error if the index is [`WindowError::OutOfBounds`]
    pub fn set_item(&self, index: usize, item: InventorySlot) -> Result<(), WindowError> {
        self.inner.set_item(index, item)
    }

    pub fn inner(&self) -> &BackingWindow {
        &self.inner
    }
}

/// Determines whether the given area will accept the given item
/// for shift-click transfer.
fn will_accept(area: Area, stack: &InventorySlot) -> bool {
    match area {
        Area::Storage => true,
        Area::CraftingOutput => false,
        Area::CraftingInput => false,
        Area::Helmet => matches!(
            stack.item_kind(),
            Some(
                ItemKind::LeatherHelmet
                    | ItemKind::ChainmailHelmet
                    | ItemKind::GoldenHelmet
                    | ItemKind::IronHelmet
                    | ItemKind::DiamondHelmet
                    | ItemKind::NetheriteHelmet
            )
        ),
        Area::Chestplate => matches!(
            stack.item_kind(),
            Some(
                ItemKind::LeatherChestplate
                    | ItemKind::ChainmailChestplate
                    | ItemKind::GoldenChestplate
                    | ItemKind::IronChestplate
                    | ItemKind::DiamondChestplate
                    | ItemKind::NetheriteChestplate
            )
        ),
        Area::Leggings => matches!(
            stack.item_kind(),
            Some(
                ItemKind::LeatherHelmet
                    | ItemKind::ChainmailLeggings
                    | ItemKind::GoldenLeggings
                    | ItemKind::IronLeggings
                    | ItemKind::DiamondLeggings
                    | ItemKind::NetheriteLeggings
            )
        ),
        Area::Boots => matches!(
            stack.item_kind(),
            Some(
                ItemKind::LeatherBoots
                    | ItemKind::ChainmailBoots
                    | ItemKind::GoldenBoots
                    | ItemKind::IronBoots
                    | ItemKind::DiamondBoots
                    | ItemKind::NetheriteBoots
            )
        ),
        Area::Hotbar => true,
        Area::Offhand => true,
        Area::FurnaceIngredient => true,
        Area::FurnaceFuel => true,
        Area::FurnaceOutput => false,
        Area::EnchantmentItem => true,
        Area::EnchantmentLapis => stack.item_kind() == Some(ItemKind::LapisLazuli),
        Area::BrewingBottle => matches!(
            stack.item_kind(),
            Some(
                ItemKind::GlassBottle
                    | ItemKind::Potion
                    | ItemKind::SplashPotion
                    | ItemKind::LingeringPotion
            )
        ),
        Area::BrewingIngredient => true,
        Area::BrewingBlazePowder => stack.item_kind() == Some(ItemKind::BlazePowder),
        Area::VillagerInput => true,
        Area::VillagerOutput => false,
        Area::BeaconPayment => matches!(
            stack.item_kind(),
            Some(
                ItemKind::IronIngot
                    | ItemKind::GoldIngot
                    | ItemKind::Diamond
                    | ItemKind::NetheriteIngot
                    | ItemKind::Emerald
            )
        ),
        Area::AnvilInput1 => true,
        Area::AnvilInput2 => true,
        Area::AnvilOutput => false,
        Area::Saddle => stack.item_kind() == Some(ItemKind::Saddle),
        Area::HorseArmor => matches!(
            stack.item_kind(),
            Some(
                ItemKind::LeatherHorseArmor
                    | ItemKind::IronHorseArmor
                    | ItemKind::GoldenHorseArmor
                    | ItemKind::DiamondHorseArmor
            )
        ),
        Area::LlamaCarpet => true,
        Area::CartographyMap => {
            matches!(stack.item_kind(), Some(ItemKind::Map | ItemKind::FilledMap))
        }
        Area::CartographyPaper => stack.item_kind() == Some(ItemKind::Paper),
        Area::CartographyOutput => false,
        Area::GrindstoneInput1 => true,
        Area::GrindstoneInput2 => true,
        Area::GrindstoneOutput => false,
        Area::LecternBook => true,
        Area::LoomBanner => true,
        Area::LoomDye => true,
        Area::LoomPattern => true,
        Area::LoomOutput => false,
        Area::StonecutterInput => true,
        Area::StonecutterOutput => false,
    }
}

/// State for a paint operation (left mouse or right mouse drag).
#[derive(Debug)]
struct PaintState {
    mouse: Mouse,
    slots: Vec<usize>,
}

impl PaintState {
    pub fn new(mouse: Mouse) -> Self {
        Self {
            mouse,
            slots: Vec::new(),
        }
    }

    pub fn add_slot(&mut self, slot: usize) -> anyhow::Result<()> {
        self.slots.push(slot);
        if self.slots.len() > 1000 {
            bail!("too many paint slots! malicious client?");
        }
        Ok(())
    }

    pub fn finish(self, window: &mut Window) -> Result<(), WindowError> {
        match self.mouse {
            Mouse::Left => self.handle_left_drag(window),
            Mouse::Right => self.handle_right_drag(window),
            Mouse::Middle => self.handle_middle_drag(window),
        }
        Ok(())
    }

    /**
        Splits cursor items evenly into every selected slot.
        Remainder of even split ends up in `window.cursor_item`.
    */
    fn handle_left_drag(&self, window: &mut Window) {
        // If the cursor has no item then there are no items to share.
        if window.cursor_item().is_empty() {
            return;
        }

        // Number of slots that can contain cursors item kind.
        let slots = self
            .slots
            .iter()
            .filter(|s| {
                // unwrap is safe because index is valid.
                let slot = &*window.inner.item(**s).unwrap();
                slot.is_mergable(window.cursor_item())
            })
            .count() as u32;

        // If slots is 0 that means there are no slots to put items into.
        // So the cursor keeps all the items.
        if slots == 0 {
            return;
        };

        let items_for_cursor = window.cursor_item().count();
        // This can't be zero because items_cursor is the count of an ItemStack and ItemStack is NonZeroU32.
        let items_per_slot = (items_for_cursor / slots).max(1);
        self.move_items_into_slots(window, items_per_slot);
    }

    /// Tries to move items_per_slot items from cursor to the slots that can contain the item
    fn move_items_into_slots(&self, window: &mut Window, items_per_slot: u32) {
        for s in &self.slots {
            let slot = &mut *window.inner.item(*s).unwrap();
            if !slot.is_mergable(window.cursor_item()) {
                continue;
            }

            window.cursor_item.transfer_to(items_per_slot, slot);
            if window.cursor_item().is_empty() {
                break;
            };
        }
    }

    fn handle_right_drag(&self, window: &mut Window) {
        self.move_items_into_slots(window, 1)
    }

    fn handle_middle_drag(&self, window: &mut Window) {
        if let Some(item) = window.cursor_item.item_kind() {
            self.move_items_into_slots(window, item.stack_size())
        }
    }
}

#[derive(Debug)]
enum Mouse {
    Left,
    Right,
    Middle,
}

#[cfg(test)]
mod tests {
    use crate::inventory::Inventory;
    use crate::items::ItemStack;

    use super::*;

    #[test]
    fn window_left_click_swap() {
        let mut window = window();

        window.left_click(0).unwrap();
        assert_eq!(window.cursor_item, Empty);

        let stack = ItemStack::new(ItemKind::Diamond, 32).unwrap();
        window
            .set_item(0, InventorySlot::Filled(stack.clone()))
            .unwrap();
        window.left_click(0).unwrap();

        assert_eq!(window.cursor_item, InventorySlot::Filled(stack.clone()));
        assert!(window.item(0).unwrap().is_empty());

        window.left_click(1).unwrap();
        assert_eq!(window.cursor_item, Empty);
        assert_eq!(*window.item(1).unwrap(), InventorySlot::Filled(stack));
    }

    #[test]
    fn window_left_click_same_item() {
        let mut window = window();

        let item = ItemStack::new(ItemKind::AcaciaSlab, 32).unwrap();
        window
            .set_item(0, InventorySlot::Filled(item.clone()))
            .unwrap();
        window.left_click(0).unwrap();

        window.set_item(1, InventorySlot::Filled(item)).unwrap();
        window.left_click(1).unwrap();

        assert_eq!(window.cursor_item, Empty);
        assert_eq!(
            *window.item(1).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::AcaciaSlab, 64).unwrap())
        );
    }

    /*
        thread 'window::tests::window_left_click_same_item' panicked at 'assertion failed: `(left == right)`
        left: `Filled(ItemStack { item: AcaciaSlab, count: 32, meta: Some(ItemStackMeta { title: "acacia_slab", lore: "", damage: None, repair_cost: None, enchantments: [] }) })`,
        right: `Filled(ItemStack { item: AcaciaSlab, count: 64, meta: Some(ItemStackMeta { title: "acacia_slab", lore: "", damage: None, repair_cost: None, enchantments: [] }) })`',
        feather/common/src/window.rs:452:9
    */

    #[test]
    fn window_right_click_pick_up_half() {
        let mut window = window();
        let stack = ItemStack::new(ItemKind::GlassPane, 17).unwrap();
        window.set_item(0, InventorySlot::Filled(stack)).unwrap();

        window.right_click(0).unwrap();
        assert_eq!(
            window.cursor_item,
            InventorySlot::Filled(ItemStack::new(ItemKind::GlassPane, 9).unwrap())
        );
        assert_eq!(
            *window.item(0).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::GlassPane, 8).unwrap())
        );
    }

    #[test]
    fn window_right_click_drop_one_item() {
        let mut window = window();
        let stack = ItemStack::new(ItemKind::GlassPane, 17).unwrap();
        window.cursor_item = InventorySlot::Filled(stack);

        window.right_click(1).unwrap();
        assert_eq!(
            window.cursor_item,
            InventorySlot::Filled(ItemStack::new(ItemKind::GlassPane, 16).unwrap())
        );
        assert_eq!(
            *window.item(1).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::GlassPane, 1).unwrap())
        );
    }

    #[test]
    fn window_right_click_swap() {
        let mut window = window();
        let stack1 = ItemStack::new(ItemKind::GlassPane, 17).unwrap();
        let stack2 = ItemStack::new(ItemKind::Diamond, 2).unwrap();
        window.cursor_item = InventorySlot::Filled(stack1.clone());
        window
            .set_item(0, InventorySlot::Filled(stack2.clone()))
            .unwrap();

        window.right_click(0).unwrap();
        assert_eq!(window.cursor_item, InventorySlot::Filled(stack2));
        assert_eq!(*window.item(0).unwrap(), InventorySlot::Filled(stack1));
    }

    #[test]
    fn window_shift_click_full_hotbar() {
        let inventory = Inventory::player();
        for i in 0..9 {
            *inventory.item(Area::Hotbar, i).unwrap() =
                InventorySlot::Filled(ItemStack::new(ItemKind::EnderPearl, 1).unwrap());
        }
        *inventory.item(Area::Storage, 0).unwrap() =
            InventorySlot::Filled(ItemStack::new(ItemKind::AcaciaSign, 1).unwrap());
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });
        let index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 0)
            .unwrap();
        window.shift_click(index).unwrap();
        assert_eq!(
            *window.item(index).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::AcaciaSign, 1).unwrap())
        );
    }

    #[test]
    fn window_shift_click_available_item_in_hotbar() {
        let inventory = Inventory::player();

        *inventory.item(Area::Hotbar, 3).unwrap() =
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 4).unwrap());
        *inventory.item(Area::Storage, 3).unwrap() =
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 7).unwrap());

        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });

        let index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 3)
            .unwrap();

        window.shift_click(index).unwrap();

        dbg!(&window);

        let hotbar_index = window
            .inner()
            .slot_to_index(&inventory, Area::Hotbar, 3)
            .unwrap();

        assert_eq!(
            *window.item(hotbar_index).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 11).unwrap())
        );
        assert!(window.item(index).unwrap().is_empty());
    }

    #[test]
    fn window_shift_click_empty_hotbar() {
        let inventory = Inventory::player();
        *inventory.item(Area::Storage, 3).unwrap() =
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 7).unwrap());
        let mut window = Window::new(BackingWindow::Player {
            player: inventory.new_handle(),
        });

        let storage_index = window
            .inner()
            .slot_to_index(&inventory, Area::Storage, 3)
            .unwrap();
        window.shift_click(storage_index).unwrap();
        let hotbar_index = window
            .inner()
            .slot_to_index(&inventory, Area::Hotbar, 0)
            .unwrap();
        assert_eq!(
            *window.item(hotbar_index).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 7).unwrap())
        );
        assert!(window.item(storage_index).unwrap().is_empty());
    }

    #[test]
    fn left_mouse_paint() {
        let mut window = window();
        window
            .set_item(
                0,
                InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 64).unwrap()),
            )
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_left_mouse_paint();
        window.add_paint_slot(0).unwrap();
        window.add_paint_slot(1).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        for &slot in &[0, 1, 5] {
            assert_eq!(
                *window.item(slot).unwrap(),
                InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 21).unwrap())
            );
        }
        assert_eq!(
            window.cursor_item,
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 1).unwrap())
        );
    }

    #[test]
    fn right_mouse_paint() {
        let mut window = window();
        window
            .set_item(
                0,
                InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 2).unwrap()),
            )
            .unwrap();
        window
            .set_item(
                4,
                InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 3).unwrap()),
            )
            .unwrap();
        window.left_click(0).unwrap();

        window.begin_right_mouse_paint();
        window.add_paint_slot(4).unwrap();
        window.add_paint_slot(5).unwrap();
        window.end_paint().unwrap();

        assert_eq!(
            *window.item(4).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 4).unwrap())
        );
        assert_eq!(
            *window.item(5).unwrap(),
            InventorySlot::Filled(ItemStack::new(ItemKind::Stone, 1).unwrap())
        );
        assert_eq!(window.cursor_item, InventorySlot::Empty);
    }

    fn window() -> Window {
        Window::new(BackingWindow::Player {
            player: Inventory::player(),
        })
    }

    #[test]
    fn set_item_test() {
        let window = window();

        window
            .set_item(45, InventorySlot::new(ItemKind::Stone, 1))
            .unwrap();
    }
}