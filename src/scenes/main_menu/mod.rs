use bevy::{app::AppExit, prelude::*};

use crate::prelude::*;

use super::SceneState;

#[allow(clippy::module_name_repetitions)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        let state = SceneState::MainMenu;

        app.add_systems(OnEnter(state), (spawn_camera, spawn_title))
            .add_systems(OnExit(state), (despawn::<Camera>, despawn::<MainMenu>))
            .add_systems(
                Update,
                (on_play_button_clicked, on_quit_button_clicked).run_if(in_state(state)),
            );

        #[cfg(feature = "dev")]
        app.register_type::<Camera>()
            .register_type::<CreditsButton>()
            .register_type::<MainMenu>()
            .register_type::<PlayButton>()
            .register_type::<QuitButton>()
            .register_type::<SettingsButton>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct CreditsButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct MainMenu;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct PlayButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct QuitButton;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct SettingsButton;

#[allow(clippy::needless_pass_by_value)]
fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera, Camera2dBundle::default()));
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Main Menu"),
            MainMenu,
            NodeBundle {
                style: Style {
                    height: Val::Vh(100.0),
                    width: Val::Vw(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    max_width: Val::Px(200.0),
                    min_width: Val::Px(20.0),
                    width: Val::Vw(6.25),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent
                .spawn(NodeBundle {
                    background_color: Color::ORANGE.with_a(0.25).into(),
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        height: Val::Vh(100.0),
                        max_width: Val::Px(460.0),
                        min_width: Val::Px(260.0),
                        width: Val::Vw(90.0),
                        padding: UiRect {
                            left: Val::Px(16.0),
                            right: Val::Px(16.0),
                            bottom: Val::Px(0.0),
                            top: Val::Px(0.0),
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            max_height: Val::Px(160.0),
                            min_height: Val::Px(20.0),
                            height: Val::Vh(100.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent.spawn((
                        Name::new("Bevy Breakout"),
                        TextBundle {
                            text: Text {
                                alignment: TextAlignment::Left,
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        color: Color::rgb(1.0, 1.0, 1.0),
                                        font: asset_server.load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                        font_size: 60.0,
                                    },
                                    value: "BEVY BREAKOUT".to_string(),
                                }],
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ));

                    parent.spawn(NodeBundle {
                        style: Style {
                            max_height: Val::Px(60.0),
                            min_height: Val::Px(0.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(16.0),
                                height: Val::Percent(100.0),
                                width: Val::Percent(100.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Name::new("Play"),
                                    PlayButton,
                                    ButtonBundle {
                                        background_color: Color::WHITE.with_a(0.0).into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            alignment: TextAlignment::Left,
                                            sections: vec![TextSection {
                                                style: TextStyle {
                                                    color: Color::rgb(1.0, 1.0, 1.0),
                                                    font: asset_server
                                                        .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                                    font_size: 36.0,
                                                },
                                                value: "PLAY".to_string(),
                                            }],
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn((
                                    Name::new("Settings"),
                                    SettingsButton,
                                    ButtonBundle {
                                        background_color: Color::WHITE.with_a(0.0).into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            alignment: TextAlignment::Left,
                                            sections: vec![TextSection {
                                                style: TextStyle {
                                                    color: Color::rgb(1.0, 1.0, 1.0),
                                                    font: asset_server
                                                        .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                                    font_size: 36.0,
                                                },
                                                value: "SETTINGS".to_string(),
                                            }],
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn((
                                    Name::new("Credits"),
                                    CreditsButton,
                                    ButtonBundle {
                                        background_color: Color::WHITE.with_a(0.0).into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            alignment: TextAlignment::Left,
                                            sections: vec![TextSection {
                                                style: TextStyle {
                                                    color: Color::rgb(1.0, 1.0, 1.0),
                                                    font: asset_server
                                                        .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                                    font_size: 36.0,
                                                },
                                                value: "CREDITS".to_string(),
                                            }],
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });

                            parent
                                .spawn((
                                    Name::new("Quit"),
                                    QuitButton,
                                    ButtonBundle {
                                        background_color: Color::WHITE.with_a(0.0).into(),
                                        ..Default::default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            alignment: TextAlignment::Left,
                                            sections: vec![TextSection {
                                                style: TextStyle {
                                                    color: Color::rgb(1.0, 1.0, 1.0),
                                                    font: asset_server
                                                        .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                                    font_size: 36.0,
                                                },
                                                value: "QUIT".to_string(),
                                            }],
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                        });
                });
        });
}

#[allow(clippy::needless_pass_by_value)]
fn on_play_button_clicked(
    mut next_state: ResMut<NextState<SceneState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            next_state.set(SceneState::Level);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn on_quit_button_clicked(
    mut event: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            event.send(AppExit);
        }
    }
}
