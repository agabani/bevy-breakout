use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct CollidablePlugin;

impl Plugin for CollidablePlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.register_type::<Collidable>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Collidable;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct CollidableBundle {
    active_events: ActiveEvents,
    collider: Collider,
    collidable: Collidable,
}

impl CollidableBundle {
    #[must_use]
    pub fn new(collider: Collider) -> Self {
        Self {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider,
            collidable: Collidable,
        }
    }
}

pub type CollisionEvents<'w, 's> = EventReader<'w, 's, CollisionEvent>;
