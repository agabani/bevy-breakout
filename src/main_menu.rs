use bevy::{app::AppExit, prelude::*};

use crate::GameState;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // camera
            .add_systems(OnEnter(GameState::MainMenu), camera_setup)
            .add_systems(OnExit(GameState::MainMenu), camera_teardown)
            // main menu
            .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
            .add_systems(OnExit(GameState::MainMenu), main_menu_teardown)
            // play button
            .add_systems(
                Update,
                play_button_interaction.run_if(in_state(GameState::MainMenu)),
            )
            // quit button
            .add_systems(
                Update,
                quit_button_interaction.run_if(in_state(GameState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub struct Camera {}

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera {}, Name::new("Camera")));
}

#[allow(clippy::needless_pass_by_value)]
fn camera_teardown(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

#[allow(clippy::needless_pass_by_value)]
fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // dimensions
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    // flex
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(8.0),
                    //
                    ..Default::default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            },
            MainMenu {},
            Name::new("Main Menu"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
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
                    PlayButton {},
                    Name::new("Play Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            alignment: TextAlignment::Center,
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            parent
                .spawn((
                    ButtonBundle {
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
                    QuitButton {},
                    Name::new("Quit Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            alignment: TextAlignment::Center,
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            )],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

#[allow(clippy::needless_pass_by_value)]
fn main_menu_teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

#[allow(clippy::needless_pass_by_value)]
fn play_button_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    if let Ok(interaction) = query.get_single() {
        match interaction {
            Interaction::Hovered | Interaction::None => {
                // TODO: change color
            }
            Interaction::Pressed => next_state.set(GameState::Level),
        };
    }
}

#[allow(clippy::needless_pass_by_value)]
fn quit_button_interaction(
    mut event_writer: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok(interaction) = query.get_single() {
        match interaction {
            Interaction::Hovered | Interaction::None => {
                // TODO: change color
            }
            Interaction::Pressed => event_writer.send(AppExit),
        }
    };
}
