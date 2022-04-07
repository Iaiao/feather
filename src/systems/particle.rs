use feather_api::prelude::*;

pub fn register(app: &mut App) {
    app.add_system(send_particle_packets);
}

fn send_particle_packets(server: Res<Server>, mut event_reader: EventReader<ParticleEvent>) {
    for particle in event_reader.iter() {
        server.broadcast_nearby_with(
            particle.world,
            particle.dimension.clone(),
            particle.position,
            |client| {
                client.send_particle(
                    &particle.particle,
                    particle.long_distance,
                    &particle.position,
                );
            },
        );
    }
}
