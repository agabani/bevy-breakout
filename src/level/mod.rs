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
            (brick::setup, camera::setup, map::setup, paddle::setup),
        )
        .add_systems(
            OnExit(GameState::Level),
            (
                brick::teardown,
                camera::teardown,
                map::teardown,
                paddle::teardown,
            ),
        )
        .add_systems(
            Update,
            (menu::escape, paddle::movement).run_if(in_state(GameState::Level)),
        );
    }
}
