use bevy::{prelude::*, window::PresentMode};

pub(crate) struct BevyConfigPlugin;

impl Plugin for BevyConfigPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                resolution: (800.0, 600.0).into(),
                title: "Bevy Breakout".into(),
                ..Default::default()
            }),
            ..Default::default()
        });

        app.add_plugins(default_plugins);
    }
}
