use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct DestructiblePlugin;

impl Plugin for DestructiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_entity_on_collision);
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Destructible;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct DestructibleBundle {
    active_events: ActiveEvents,
    destructible: Destructible,
}

impl DestructibleBundle {
    #[must_use]
    pub fn new() -> DestructibleBundle {
        DestructibleBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
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
    mut events: EventReader<CollisionEvent>,
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
