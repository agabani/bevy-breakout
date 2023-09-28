use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
};

use crate::prelude::*;

use super::SceneState;

#[allow(clippy::module_name_repetitions)]
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        let state = SceneState::Title;

        app.add_systems(OnEnter(state), (spawn_camera, spawn_title))
            .add_systems(OnExit(state), (despawn::<Camera>, despawn::<Title>))
            .add_systems(
                Update,
                (
                    interaction::<KeyboardInput>,
                    interaction::<MouseButtonInput>,
                    interaction::<TouchInput>,
                )
                    .run_if(in_state(state)),
            );

        #[cfg(feature = "dev")]
        app.register_type::<Camera>().register_type::<Title>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Title;

#[allow(clippy::needless_pass_by_value)]
fn despawn<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn interaction<T: Event>(
    mut next_state: ResMut<NextState<SceneState>>,
    mut events: EventReader<T>,
) {
    for _events in &mut events {
        next_state.set(SceneState::MainMenu);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera, Camera2dBundle::default()));
}

#[allow(clippy::needless_pass_by_value)]
fn spawn_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Title"),
            Title,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect {
                        top: Val::Vh(40.0),
                        bottom: Val::Vh(10.0),
                        ..Default::default()
                    },
                    width: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Bevy Breakout"),
                TextBundle {
                    text: Text {
                        alignment: TextAlignment::Center,
                        sections: vec![TextSection {
                            style: TextStyle {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                font: asset_server.load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                font_size: 84.0,
                            },
                            value: "BEVY BREAKOUT".to_string(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));

            parent.spawn((
                Name::new("Press any button to start"),
                TextBundle {
                    text: Text {
                        alignment: TextAlignment::Center,
                        sections: vec![TextSection {
                            style: TextStyle {
                                color: Color::rgb(1.0, 1.0, 1.0),
                                font: asset_server.load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                font_size: 24.0,
                            },
                            value: "PRESS ANY BUTTON TO START".to_string(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        });
}
