//! Sends tablist info to clients via the Player Info packet.
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(remove_tablist_players)
        .add_system(add_tablist_players)
        .add_system(change_tablist_player_gamemode);
}

fn remove_tablist_players(
    server: Res<Server>,
    query: Query<&Uuid, With<Player>>,
    mut event_reader: EventReader<EntityEvent<EntityRemoveEvent>>,
) {
    for event in event_reader.iter() {
        if let Ok(uuid) = query.get(event.entity) {
            server.broadcast_with(|client| client.remove_tablist_player(**uuid));
        }
    }
}

fn add_tablist_players(
    server: Res<Server>,
    query: Query<
        (
            &ClientId,
            &Uuid,
            &PlayerName,
            &Gamemode,
            &External<Vec<ProfileProperty>>,
        ),
        With<Player>,
    >,
    joined_query: Query<
        (
            &ClientId,
            &Uuid,
            &PlayerName,
            &Gamemode,
            &External<Vec<ProfileProperty>>,
        ),
        Added<Player>,
    >,
) {
    for (&client_id, uuid, name, &gamemode, profile) in joined_query.iter() {
        // Add this player to other players' tablists
        server.broadcast_with(|client| {
            client.add_tablist_player(**uuid, name.to_string(), profile, gamemode)
        });

        // Add other players to this player's tablist
        for (_, other_uuid, name, &gamemode, profile) in query.iter() {
            if let Some(client) = server.client(client_id) {
                if **other_uuid != **uuid {
                    client.add_tablist_player(**other_uuid, name.to_string(), profile, gamemode);
                }
            }
        }
    }
}

fn change_tablist_player_gamemode(
    server: Res<Server>,
    query: Query<&Uuid>,
    mut event_reader: EventReader<EntityEvent<GamemodeChangeEvent>>,
) {
    for event in event_reader.iter() {
        // Change this player's gamemode in players' tablists
        let uuid = query.get(event.entity).unwrap();
        server.broadcast_with(|client| client.change_player_tablist_gamemode(**uuid, ***event));
    }
}
