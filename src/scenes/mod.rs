pub mod level;
pub mod main_menu;
pub mod title;

use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_state::<SceneState>().add_plugins((
            level::LevelPlugin,
            main_menu::MainMenuPlugin,
            title::TitlePlugin,
        ));
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash, States)]
pub enum SceneState {
    Level,
    MainMenu,
    #[default]
    Title,
}
