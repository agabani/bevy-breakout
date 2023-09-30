#![warn(clippy::pedantic)]

pub mod bevy_config;
pub mod components;
#[cfg(feature = "dev")]
pub mod dev;
pub(crate) mod math;
pub mod physics;
pub mod prelude;
pub mod scenes;
pub mod subsystems;

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
            SubsystemsPlugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(DevPlugin);
    }
}
