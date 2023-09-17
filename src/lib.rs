#![warn(clippy::pedantic)]

pub(crate) mod level;
pub(crate) mod main_menu;
pub(crate) mod math;
pub mod plugins;

use bevy::prelude::*;

use crate::prelude::*;
use level::LevelPlugin;
use main_menu::MainMenuPlugin;

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
            .add_plugins(BevyPlugin)
            .add_plugins(LevelPlugin)
            .add_plugins(MainMenuPlugin)
            .add_plugins(PhysicsPlugin)
            .add_plugins(LookAtPlugin)
            .add_plugins(OrbitControllerPlugin);
        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}

pub mod prelude {
    #[cfg(feature = "dev")]
    pub use crate::plugins::dev::*;
    pub use crate::plugins::{bevy::*, look_at::*, orbit_controller::*, physics::*};
}
