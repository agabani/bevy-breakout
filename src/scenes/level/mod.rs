pub(crate) mod ball;
pub(crate) mod brick;
pub(crate) mod camera;
pub(crate) mod map;
pub(crate) mod menu;
pub(crate) mod paddle;
pub(crate) mod scoreboard;

use bevy::prelude::*;

use crate::{prelude::*, scenes::SceneState};

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SceneState::Level),
            (
                ball::setup,
                brick::setup,
                camera::setup,
                map::setup,
                paddle::setup,
                scoreboard::setup,
                reset_score,
            ),
        )
        .add_systems(
            OnExit(SceneState::Level),
            (
                ball::teardown,
                brick::teardown,
                camera::teardown,
                map::teardown,
                paddle::teardown,
                scoreboard::teardown,
            ),
        )
        .add_systems(
            Update,
            (menu::escape, scoreboard::update).run_if(in_state(SceneState::Level)),
        );
    }
}

fn reset_score(mut resource: ResMut<Score>) {
    resource.reset();
}
