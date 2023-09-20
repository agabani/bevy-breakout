use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct VelocitizedPlugin;

impl Plugin for VelocitizedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity);

        #[cfg(feature = "dev")]
        app.register_type::<Velocitized>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Velocitized {
    linear: f32,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct VelocitizedBundle {
    velocitized: Velocitized,
    velocity: Velocity,
}

impl VelocitizedBundle {
    #[must_use]
    pub fn new(linear: f32, direction: Vec2) -> VelocitizedBundle {
        VelocitizedBundle {
            velocitized: Velocitized { linear },
            velocity: Velocity {
                linvel: direction.normalize() * linear,
                ..Default::default()
            },
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn update_velocity(mut query: Query<(&mut Velocity, &Velocitized)>) {
    for (mut velocity, velocitized) in &mut query {
        velocity.linvel = velocity.linvel.normalize() * velocitized.linear;
    }
}
