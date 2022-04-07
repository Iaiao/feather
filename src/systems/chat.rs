use feather_api::bevy::ecs as bevy_ecs;
use feather_api::prelude::*;

/// Marker component for the console entity.
#[derive(Component)]
struct Console;

pub fn register(app: &mut App) {
    app.add_startup_system(create_console)
        .add_system(flush_console_chat_box)
        .add_system(flush_chat_boxes);
}

/// Flushes players' chat mailboxes and sends the needed packets.
fn flush_chat_boxes(server: Res<Server>, mut query: Query<(&mut ChatBox, &ClientId)>) {
    for (mut mailbox, client_id) in query.iter_mut() {
        if let Some(client) = server.client(*client_id) {
            for (sender, message) in mailbox.drain() {
                if let Some(sender) = sender {
                    client.send_chat_message(sender, message);
                } else {
                    client.send_message(message);
                }
            }
            for title in mailbox.drain_titles() {
                client.send_title(title);
            }
        }
    }
}

/// Prints chat messages to the console.
fn flush_console_chat_box(mut query: Query<&mut ChatBox, With<Console>>) {
    let mut mailbox = query.single_mut();
    for (_sender_uuid, message) in mailbox.drain() {
        // TODO: properly display chat message
        log::info!("{:?}", message.text());
    }
}

fn create_console(mut commands: Commands) {
    // Create the console entity so the console can receive messages
    commands
        .spawn()
        .insert(Console)
        .insert(ChatBox::new(ChatPreference::All));
}
