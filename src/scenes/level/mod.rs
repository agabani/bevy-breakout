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
                change_background_music,
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

fn change_background_music(mut play: EventWriter<background_music::PlayEvent>) {
    play.send(background_music::PlayEvent::new(
        ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD.path(),
    ));
}
