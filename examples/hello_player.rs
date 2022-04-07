extern crate feather_api;

use feather_api::prelude::*;

#[derive(DynamicPlugin)]
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello_world).add_system(hello_player);
    }

    fn name(&self) -> &str {
        "my_plugin"
    }
}

fn hello_world() {
    log::info!("Hello world");
}

fn hello_player(mut query: Query<(&PlayerName, &mut ChatBox), Added<Player>>) {
    for (name, mut chat_box) in query.iter_mut() {
        chat_box.send_system(format!("Hello {}", name));
    }
}
