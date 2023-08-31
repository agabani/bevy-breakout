pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod main_menu;

use bevy::prelude::*;
use bevy_config::BevyConfig;
#[cfg(feature = "dev")]
use dev::DevPlugin;

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins(BevyConfig);
        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}
