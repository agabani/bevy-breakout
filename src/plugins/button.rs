use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interaction);

        #[cfg(feature = "dev")]
        app.register_type::<Button>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct Button;

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct ButtonBundle {
    button: Button,
    button_bundle: bevy::prelude::ButtonBundle,
}

impl ButtonBundle {
    #[must_use]
    pub fn new() -> Self {
        Self {
            button: Button,
            button_bundle: bevy::prelude::ButtonBundle {
                style: Style {
                    height: Val::Px(80.0),
                    width: Val::Px(200.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..Default::default()
            },
        }
    }
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
pub fn interaction(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in &mut query {
        match interaction {
            Interaction::Hovered => background_color.0 = Color::rgb(0.5, 0.5, 0.5),
            Interaction::None => background_color.0 = Color::rgb(0.3, 0.3, 0.3),
            Interaction::Pressed => {}
        };
    }
}
