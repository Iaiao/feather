use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub env: PhysicalEnvironment,
}

impl Velocity {
    pub fn apply_x_z(&mut self, position: &mut Position) {
        position.x += self.x;
        position.z += self.x;
    }

    pub fn apply_y(&mut self, position: &mut Position) {
        position.y += self.y;
    }

    pub fn pre_gravity_tick(&mut self, on_ground: OnGround) {
        if !*on_ground {
            self.y -= self.env.gravity_acceleration;
        }
    }
    pub fn post_gravity_tick(&mut self, on_ground: OnGround) {
        let environment_resistance = if *on_ground {
            self.env.solid_block_resistance
        } else {
            self.env.air_resistance
        };
        self.x *= 1.0 - environment_resistance;
        self.y *= 1.0 - environment_resistance;
        self.z *= 1.0 - environment_resistance;
    }

    pub fn default_living() -> Self {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            env: PhysicalEnvironment::living(),
        }
    }

    pub fn default_tnt() -> Self {
        const TNT_UP_MOVE: f64 = 0.2;
        const TNT_DISTRIBUTION_RADIUS: f64 = 0.02;

        let angle = std::f64::consts::PI * 2.0 * rand::random::<f64>();
        Velocity {
            x: angle.sin() * TNT_DISTRIBUTION_RADIUS,
            y: TNT_UP_MOVE,
            z: angle.cos() * TNT_DISTRIBUTION_RADIUS,
            env: PhysicalEnvironment::falling_block(),
        }
    }

    pub fn static_tnt() -> Self {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            env: PhysicalEnvironment::falling_block(),
        }
    }

    pub fn default_block() -> Self {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            env: PhysicalEnvironment::falling_block(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Component)]
pub struct PhysicalEnvironment {
    air_resistance: f64,
    solid_block_resistance: f64,
    // TODO water, lava resistance
    gravity_acceleration: f64,
}

impl PhysicalEnvironment {
    pub fn falling_block() -> Self {
        PhysicalEnvironment {
            air_resistance: 0.02,
            solid_block_resistance: 0.3,
            gravity_acceleration: 0.04,
        }
    }

    pub fn living() -> Self {
        PhysicalEnvironment {
            air_resistance: 0.01,
            solid_block_resistance: 1.0,
            gravity_acceleration: 0.08,
        }
    }
}
