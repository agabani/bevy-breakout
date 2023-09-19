use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct DestructiblePlugin;

impl Plugin for DestructiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_entity_on_collision);

        #[cfg(feature = "dev")]
        app.register_type::<Destructible>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Destructible;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct DestructibleBundle {
    destructible: Destructible,
}

impl DestructibleBundle {
    #[must_use]
    pub fn new() -> DestructibleBundle {
        DestructibleBundle {
            destructible: Destructible,
        }
    }
}

impl Default for DestructibleBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::needless_pass_by_value)]
fn despawn_entity_on_collision(
    mut commands: Commands,
    mut events: CollisionEvents,
    query: Query<Entity, With<Destructible>>,
) {
    for event in &mut events {
        if let CollisionEvent::Stopped(a, b, _) = event {
            if let Ok(entity) = query.get(*a) {
                commands.entity(entity).despawn_recursive();
            }
            if let Ok(entity) = query.get(*b) {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
