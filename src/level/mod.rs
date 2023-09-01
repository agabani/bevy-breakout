pub(crate) mod camera;
pub(crate) mod menu;
pub(crate) mod paddle;

use bevy::prelude::*;

use crate::GameState;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // camera
            .add_systems(OnEnter(GameState::Level), camera::setup)
            .add_systems(OnExit(GameState::Level), camera::teardown)
            // paddle
            .add_systems(OnEnter(GameState::Level), paddle::setup)
            .add_systems(OnExit(GameState::Level), paddle::teardown)
            .add_systems(Update, paddle::movement.run_if(in_state(GameState::Level)))
            // menu
            .add_systems(Update, menu::escape.run_if(in_state(GameState::Level)));
    }
}
