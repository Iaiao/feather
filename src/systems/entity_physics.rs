use feather_api::common::velocity::Velocity;
use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(velocity.label("velocity").after("send_entity_movement"))
        .add_system(update_onground.label("update_onground").after("velocity"))
        .add_system(gravity.after("update_onground"));
}

fn velocity(mut query: Query<(&mut Position, &mut Velocity, &EntityKind)>) {
    for (mut position, mut velocity, kind) in query.iter_mut() {
        if !matches!(
            kind,
            EntityKind::ExperienceOrb | EntityKind::Painting | EntityKind::Player
        ) {
            velocity.apply_x_z(&mut *position);
        }
    }
}

fn update_onground(
    mut query: Query<(&Position, &EntityDimension, &EntityWorld, &mut OnGround)>,
    worlds: Query<&Dimensions>,
) {
    for (position, dimension, world, mut on_ground) in query.iter_mut() {
        let dimension = worlds.get(***world).unwrap().get(&*dimension).unwrap();
        let new_on_ground = dimension
            .block_at(position.add(0.0, -f64::MIN_POSITIVE, 0.0).block().down())
            .map_or(false, |block| block.is_solid());
        if **on_ground != new_on_ground {
            **on_ground = new_on_ground
        }
    }
}

fn gravity(mut query: Query<(&mut Position, &mut Velocity, &OnGround, &EntityKind)>) {
    for (mut position, mut velocity, on_ground, kind) in query.iter_mut() {
        if !matches!(
            kind,
            EntityKind::ExperienceOrb | EntityKind::Painting | EntityKind::Player
        ) {
            velocity.pre_gravity_tick(*on_ground);
            velocity.apply_y(&mut *position);
            velocity.post_gravity_tick(*on_ground);
        }
    }
}
