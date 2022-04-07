use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(send_plugin_message_packets);
}

fn send_plugin_message_packets(
    server: Res<Server>,
    query: Query<&ClientId>,
    mut event_reader: EventReader<EntityEvent<PluginMessageEvent>>,
) {
    for event in event_reader.iter() {
        let client_id = query.get(event.entity).unwrap();
        if let Some(client) = server.client(*client_id) {
            client.send_plugin_message(event.channel.clone(), event.data.clone());
        }
    }
}
