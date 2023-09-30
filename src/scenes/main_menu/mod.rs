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
                (
                    on_click_next_state::<CreditsButton>(SceneState::Credits),
                    on_click_next_state::<PlayButton>(SceneState::Level),
                    on_click_next_state::<SettingsButton>(SceneState::Settings),
                    on_click_quit,
                )
                    .run_if(in_state(state)),
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

fn on_click_next_state<T: Component>(
    scene_state: SceneState,
) -> impl Fn(ResMut<NextState<SceneState>>, Query<&Interaction, (Changed<Interaction>, With<T>)>) {
    move |mut next_state, query| {
        for &interaction in &query {
            if interaction == Interaction::Pressed {
                next_state.set(scene_state);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn on_click_quit(
    mut event: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            event.send(AppExit);
        }
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
                            template_spawn_button(
                                parent,
                                &asset_server,
                                "Play",
                                "PLAY".to_string(),
                                PlayButton,
                            );

                            template_spawn_button(
                                parent,
                                &asset_server,
                                "Settings",
                                "SETTINGS".to_string(),
                                SettingsButton,
                            );

                            template_spawn_button(
                                parent,
                                &asset_server,
                                "Credits",
                                "CREDITS".to_string(),
                                CreditsButton,
                            );

                            #[cfg(not(target_family = "wasm"))]
                            template_spawn_button(
                                parent,
                                &asset_server,
                                "Quit",
                                "QUIT".to_string(),
                                QuitButton,
                            );
                        });
                });
        });
}

fn template_spawn_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    name: impl Into<std::borrow::Cow<'static, str>>,
    value: String,
    component: impl Component,
) {
    parent
        .spawn((
            Name::new(name),
            component,
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
                            font: asset_server.load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                            font_size: 36.0,
                        },
                        value,
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
