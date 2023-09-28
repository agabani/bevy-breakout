#![warn(clippy::pedantic)]

pub mod bevy_config;
pub mod components;
#[cfg(feature = "dev")]
pub mod dev;
pub(crate) mod math;
pub mod physics;
pub mod prelude;
pub mod scenes;

use bevy::prelude::*;

use crate::{prelude::*, scenes::ScenesPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BevyDefaultPlugin,
            ComponentsPlugin,
            PhysicsPlugin,
            ScenesPlugin,
        ));

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
