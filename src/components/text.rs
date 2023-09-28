use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.register_type::<Text>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Text;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct TextBundle {
    text: Text,
    text_bundle: bevy::prelude::TextBundle,
}

impl TextBundle {
    #[must_use]
    pub fn new(asset_server: &Res<AssetServer>, value: impl Into<String>) -> Self {
        Self {
            text: Text,
            text_bundle: bevy::prelude::TextBundle {
                text: bevy::prelude::Text {
                    alignment: TextAlignment::Center,
                    sections: vec![TextSection::new(
                        value,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    )],
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
