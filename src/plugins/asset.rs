use std::borrow::Cow;

use bevy::prelude::*;

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        #[cfg(feature = "dev")]
        app.register_type::<Asset>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Asset<'a> {
    path: Cow<'a, str>,
}

impl<'a> Asset<'a> {
    pub fn path(&'a self) -> &'a str {
        &self.path
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct AssetBundle {
    asset: Asset<'static>,
    license: License<'static>,
    name: Name,
}

impl AssetBundle {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        asset: Asset<'static>,
        license: License<'static>,
    ) -> Self {
        Self {
            asset,
            license,
            name: Name::new(name),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(AssetBundle::new(
        "Asset: Sounds: Of Far Different Nature - Force Field",
        ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD,
        License::new(
            Cow::Borrowed("Creative Commons Attribution 4.0 International Public License"),
            Cow::Borrowed("Music: 'Force Field' by Of Far Different Nature (https://fardifferent.bandcamp.com/)"),
        )
    ));
}

pub const ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD: Asset = Asset {
    path: Cow::Borrowed(
        "sounds/Of Far Different Nature - Force Field/Of Far Different Nature - Force Field (CC-BY).ogg",
    ),
};