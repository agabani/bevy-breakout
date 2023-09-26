#![warn(clippy::pedantic)]

pub(crate) mod level;
pub(crate) mod main_menu;
pub(crate) mod math;
pub mod plugins;

use bevy::prelude::*;
use prelude::ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD;

pub mod prelude {
    pub use crate::plugins::{
        asset::*, bevy::*, button::*, collidable::*, destructible::*, license::*, look_at::*,
        orbit_controller::*, physics::*, scorable::*, text::*, velocitized::*,
    };

    #[cfg(feature = "dev")]
    pub use crate::plugins::dev::*;
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Hash, States)]
pub enum GameState {
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
                AssetPlugin,
                BevyDefaultPlugin,
                ButtonPlugin,
                DestructiblePlugin,
                LicensePlugin,
                LookAtPlugin,
                OrbitControllerPlugin,
                PhysicsPlugin,
                ScorablePlugin,
                TextPlugin,
                VelocitizedPlugin,
            ))
            .add_plugins((LevelPlugin, MainMenuPlugin));

        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);

        app.add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load(ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD.path()),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.3)),
            ..default()
        },
        ..default()
    });
}
