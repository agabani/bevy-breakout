use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Minimum z-value for the physics engine to respect rotation.
///
/// Any positive non 0 value will suffice.
/// Using value 1.0 because it's probably an easy number for calculations.
pub(crate) const MIN_Z: f32 = 1.0;

#[allow(clippy::module_name_repetitions)]
pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
