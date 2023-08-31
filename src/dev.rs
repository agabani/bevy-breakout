use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
