pub mod level;
pub mod main_menu;

use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_state::<SceneState>()
            .add_plugins((level::LevelPlugin, main_menu::MainMenuPlugin));
    }
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
pub enum SceneState {
    #[default]
    MainMenu,
    Level,
}
