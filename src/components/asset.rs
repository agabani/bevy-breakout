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
    #[must_use]
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

pub const ASSET_FONT_FIRA_SANS_BOLD: Asset = Asset {
    path: std::borrow::Cow::Borrowed("fonts/FiraSans-Bold.ttf"),
};

pub const ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_ETHNIC_BEAT: Asset = Asset {
    path: Cow::Borrowed(
        "sounds/Of Far Different Nature - Ethnic Beat/Of Far Different Nature - Ethnic Beat (CC-BY).ogg",
    ),
};

pub const ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD: Asset = Asset {
    path: Cow::Borrowed(
        "sounds/Of Far Different Nature - Force Field/Of Far Different Nature - Force Field (CC-BY).ogg",
    ),
};

fn setup(mut commands: Commands) {
    commands.spawn(AssetBundle::new(
        "Asset: Font: FiraSans-Bold",
        ASSET_FONT_FIRA_SANS_BOLD,
        License::new(
            Cow::Borrowed("SIL OPEN FONT LICENSE Version 1.1"),
            Cow::Borrowed("Font: FiraSans-Bold"),
        ),
    ));

    commands.spawn(AssetBundle::new(
        "Asset: Sound: Of Far Different Nature - Ethnic Beat",
        ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_ETHNIC_BEAT,
        License::new(
            Cow::Borrowed("Creative Commons Attribution 4.0 International Public License"),
            Cow::Borrowed("Music: 'Ethnic Beat' by Of Far Different Nature (https://fardifferent.bandcamp.com/)"),
        )
    ));

    commands.spawn(AssetBundle::new(
        "Asset: Sound: Of Far Different Nature - Force Field",
        ASSET_SOUND_OF_FAR_DIFFERENT_NATURE_FORCE_FIELD,
        License::new(
            Cow::Borrowed("Creative Commons Attribution 4.0 International Public License"),
            Cow::Borrowed("Music: 'Force Field' by Of Far Different Nature (https://fardifferent.bandcamp.com/)"),
        )
    ));
}
