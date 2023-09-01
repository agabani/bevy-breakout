use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
