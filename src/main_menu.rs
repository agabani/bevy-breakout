use bevy::prelude::*;

use crate::GameState;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu);
    }
}

pub(crate) fn setup_menu() {}
