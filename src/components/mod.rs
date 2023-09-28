pub mod asset;
pub mod button;
pub mod collidable;
pub mod destructible;
pub mod license;
pub mod look_at;
pub mod orbit_controller;
pub mod scorable;
pub mod text;
pub mod velocitized;

use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AssetPlugin,
            ButtonPlugin,
            CollidablePlugin,
            DestructiblePlugin,
            LicensePlugin,
            LookAtPlugin,
            OrbitControllerPlugin,
            ScorablePlugin,
            TextPlugin,
            VelocitizedPlugin,
        ));
    }
}
