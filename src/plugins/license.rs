use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct LicensePlugin;

impl Plugin for LicensePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.register_type::<License>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct License;
