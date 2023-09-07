#![warn(clippy::pedantic)]

pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod level;
pub(crate) mod main_menu;
pub(crate) mod math;
pub(crate) mod physics;

use bevy::prelude::*;

use bevy_config::BevyConfigPlugin;
#[cfg(feature = "dev")]
use dev::DevPlugin;
use level::LevelPlugin;
use main_menu::MainMenuPlugin;
use physics::PhysicsPlugin;

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Level,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(BevyConfigPlugin)
            .add_plugins(LevelPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(PhysicsPlugin);
        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}
