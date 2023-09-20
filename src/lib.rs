#![warn(clippy::pedantic)]

pub(crate) mod level;
pub(crate) mod main_menu;
pub(crate) mod math;
pub mod plugins;

use bevy::prelude::*;

pub mod prelude {
    pub use crate::plugins::{
        bevy::*, collidable::*, destructible::*, look_at::*, orbit_controller::*, physics::*,
        scorable::*, velocitized::*,
    };

    #[cfg(feature = "dev")]
    pub use crate::plugins::dev::*;
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Level,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        use crate::prelude::*;

        use level::LevelPlugin;
        use main_menu::MainMenuPlugin;

        app.add_state::<GameState>()
            .add_plugins((
                BevyDefaultPlugin,
                DestructiblePlugin,
                LookAtPlugin,
                OrbitControllerPlugin,
                PhysicsPlugin,
                ScorablePlugin,
                VelocitizedPlugin,
            ))
            .add_plugins((LevelPlugin, MainMenuPlugin));

        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}
