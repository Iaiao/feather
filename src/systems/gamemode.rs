use feather_api::base::anvil::player::PlayerAbilities;
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_event::<EntityEvent<GamemodeChangeEvent>>()
        .add_system(gamemode_change);
}

fn gamemode_change(
    server: Res<Server>,
    mut query: Query<(
        &ClientId,
        &WalkSpeed,
        &CreativeFlyingSpeed,
        &mut CanCreativeFly,
        &mut CreativeFlying,
        &mut Instabreak,
        &mut CanBuild,
        &mut Invulnerable,
        &mut Gamemode,
        &mut PreviousGamemode,
    )>,
    mut events: EventReader<EntityEvent<GamemodeChangeEvent>>,
) {
    for event in events.iter() {
        let (
            &client_id,
            &walk_speed,
            &fly_speed,
            mut may_fly,
            mut is_flying,
            mut instabreak,
            mut may_build,
            mut invulnerable,
            mut gamemode,
            mut prev_gamemode,
        ) = query.get_mut(event.entity).unwrap();
        if ***event == *gamemode {
            continue;
        }
        *prev_gamemode = PreviousGamemode(Some(*gamemode));
        *gamemode = ***event;
        match *gamemode {
            Gamemode::Creative => {
                if !**instabreak {
                    instabreak.0 = true;
                }
                if !**may_fly {
                    may_fly.0 = true;
                }
                if !**may_build {
                    may_build.0 = true;
                }
                if !**invulnerable {
                    invulnerable.0 = true;
                }
            }
            Gamemode::Spectator => {
                if !**is_flying {
                    is_flying.0 = true;
                }
                if **instabreak {
                    instabreak.0 = false;
                }
                if !**may_fly {
                    may_fly.0 = true;
                }
                if **may_build {
                    may_build.0 = false;
                }
                if !**invulnerable {
                    invulnerable.0 = true;
                }
            }
            Gamemode::Survival => {
                if **is_flying {
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly.0 = false;
                }
                if !**may_build {
                    may_build.0 = true;
                }
                if **invulnerable {
                    invulnerable.0 = false;
                }
            }
            Gamemode::Adventure => {
                if **is_flying {
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly.0 = false;
                }
                if **may_build {
                    may_build.0 = false;
                }
                if **invulnerable {
                    invulnerable.0 = false;
                }
            }
        }
        server.client(client_id).unwrap().change_gamemode(*gamemode);
        server
            .client(client_id)
            .unwrap()
            .send_abilities(&PlayerAbilities {
                walk_speed,
                fly_speed,
                may_fly: *may_fly,
                is_flying: *is_flying,
                may_build: *may_build,
                instabreak: *instabreak,
                invulnerable: *invulnerable,
            });
    }
}
