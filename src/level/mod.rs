pub(crate) mod ball;
pub(crate) mod brick;
pub(crate) mod camera;
pub(crate) mod map;
pub(crate) mod menu;
pub(crate) mod paddle;

use bevy::prelude::*;

use crate::GameState;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Level),
            (
                ball::setup,
                brick::setup,
                camera::setup,
                map::setup,
                paddle::setup,
            ),
        )
        .add_systems(
            OnExit(GameState::Level),
            (
                ball::teardown,
                brick::teardown,
                camera::teardown,
                map::teardown,
                paddle::teardown,
            ),
        )
        .add_systems(
            Update,
            (
                ball::velocity,
                brick::collision_ball,
                menu::escape,
                paddle::movement_keyboard,
                paddle::movement_touches,
            )
                .run_if(in_state(GameState::Level)),
        );
    }
}
