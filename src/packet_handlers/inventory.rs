use feather_api::prelude::Window;
use feather_api::prelude::*;
use feather_api::protocol::packets::client::{
    ClickWindow, CreativeInventoryAction, WindowClickMode,
};

use crate::packet_handlers::Disconnect;

pub fn handle_creative_inventory_action(
    packet: CreativeInventoryAction,
    server: &Server,
    gamemode: Gamemode,
    window: &mut Window,
    client_id: ClientId,
) -> Result<(), Disconnect> {
    if gamemode != Gamemode::Creative {
        return Err("cannot use Creative Inventory Action outside of creative mode".to_string());
    }

    if packet.slot != -1 {
        if !matches!(window.inner(), BackingWindow::Player { .. }) {
            return Err("cannot use Creative Inventory Action in external inventories".to_owned());
        }

        if packet.clicked_item.is_empty() {
            let item = window
                .item(packet.slot as usize)
                .map_err(|err| err.to_string())?
                .clone();
            window.set_cursor_item(item);
        } else {
            window.set_cursor_item(InventorySlot::Empty)
        }
        window
            .inner()
            .set_item(packet.slot as usize, packet.clicked_item)
            .map_err(|err| err.to_string())?;

        // Sends the client updates about window changes.
        // Is required to make delete inventory button reflect in-game.
        let client = server.client(client_id).unwrap();
        client.send_window_items(window);
    }
    Ok(())
}

pub fn handle_click_window(
    server: &Server,
    packet: ClickWindow,
    client_id: ClientId,
    window: &mut Window,
) -> Result<(), Disconnect> {
    _handle_click_window(&packet, window)?;

    let client = server.client(client_id).unwrap();

    if packet.slot >= 0 {
        let item = window
            .item(packet.slot as usize)
            .map_err(|err| err.to_string())?
            .clone();
        let old_cursor_item = window.cursor_item();
        client.send_inventory_slot(packet.slot, old_cursor_item);
        window.set_cursor_item(item);
    }
    client.send_cursor_slot(window.cursor_item());

    client.send_window_items(&*window);

    Ok(())
}

fn _handle_click_window(packet: &ClickWindow, window: &mut Window) -> Result<(), Disconnect> {
    println!("{:?}", packet.mode);
    match packet.mode {
        WindowClickMode::Click => match packet.button {
            0 => window
                .left_click(packet.slot as usize)
                .map_err(|err| err.to_string())?,
            1 => window
                .right_click(packet.slot as usize)
                .map_err(|err| err.to_string())?,
            _ => return Err("Unrecognized click".to_string()),
        },
        WindowClickMode::ShiftClick => match packet.button {
            0 | 1 => window
                .shift_click(packet.slot as usize)
                .map_err(|err| err.to_string())?,
            _ => return Err("Unrecognized shift click".to_string()),
        },
        // TODO implement other click types
        WindowClickMode::Drag => match packet.button {
            0 => window.begin_left_mouse_paint(),
            4 => window.begin_right_mouse_paint(),
            8 => window.begin_middle_mouse_paint(),
            1 | 5 | 9 => window
                .add_paint_slot(packet.slot as usize)
                .map_err(|err| err.to_string())?,
            2 | 6 | 10 => window.end_paint().map_err(|err| err.to_string())?,
            _ => return Err("Unrecognized paint operation".to_owned()),
        },
        _ => return Err("Unrecognized window click mode".to_owned()),
    };
    Ok(())
}
