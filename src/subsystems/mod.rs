pub mod background_music;

use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct SubsystemsPlugin;

impl Plugin for SubsystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(background_music::Plugin);
    }
}
