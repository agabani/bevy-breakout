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

#[allow(clippy::needless_pass_by_value)]
pub fn interaction(query: Query<&Interaction, (Changed<Interaction>, With<Button>)>) {
    for interaction in &query {
        match interaction {
            Interaction::Hovered => {
                // println!("Hovered");
            }
            Interaction::None => {
                // println!("None");
            }
            Interaction::Pressed => {
                // println!("Pressed");
            }
        };
    }
}
