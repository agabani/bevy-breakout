use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ScorablePlugin;

impl Plugin for ScorablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Update, increase_score_on_collision);

        #[cfg(feature = "dev")]
        app.register_type::<Scorable>().register_type::<Score>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Scorable;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct ScorableBundle {
    destructible: Scorable,
}

impl ScorableBundle {
    #[must_use]
    pub fn new() -> ScorableBundle {
        ScorableBundle {
            destructible: Scorable,
        }
    }
}

impl Default for ScorableBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Resource)]
#[cfg_attr(feature = "dev", derive(Reflect))]
#[cfg_attr(feature = "dev", reflect(Resource))]
pub struct Score {
    value: usize,
}

#[allow(clippy::needless_pass_by_value)]
fn increase_score_on_collision(
    mut score: ResMut<Score>,
    mut events: CollisionEvents,
    query: Query<Entity, With<Scorable>>,
) {
    for event in &mut events {
        if let CollisionEvent::Stopped(a, b, _) = event {
            if let Ok(_entity) = query.get(*a) {
                score.value += 1;
            }
            if let Ok(_entity) = query.get(*b) {
                score.value += 1;
            }
        }
    }
}
