pub(crate) mod bevy_config;

use bevy::prelude::*;
use bevy_config::BevyConfig;

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins(BevyConfig);
    }
}
