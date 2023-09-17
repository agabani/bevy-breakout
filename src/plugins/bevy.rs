use bevy::{prelude::*, window::PresentMode};

#[allow(clippy::module_name_repetitions)]
pub struct BevyDefaultPlugin;

impl Plugin for BevyDefaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                resolution: (800.0, 600.0).into(),
                title: "Bevy Breakout".into(),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct BevyMinimalPlugin;

impl Plugin for BevyMinimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MinimalPlugins);
    }
}
