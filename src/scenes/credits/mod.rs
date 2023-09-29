use bevy::prelude::*;

use crate::prelude::*;

use super::SceneState;

#[allow(clippy::module_name_repetitions)]
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        let state = SceneState::Credits;

        app.add_systems(OnEnter(state), (spawn_camera, spawn_credits))
            .add_systems(OnExit(state), (despawn::<Camera>, despawn::<Credits>));

        #[cfg(feature = "dev")]
        app.register_type::<Camera>().register_type::<Credits>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Camera;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct Credits;

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
fn spawn_credits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&License<'static>>,
) {
    commands
        .spawn((
            Name::new("Credits"),
            Credits,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    row_gap: Val::Px(12.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            for license in &query {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            Name::new(license.text().to_string()),
                            TextBundle {
                                text: Text {
                                    alignment: TextAlignment::Left,
                                    sections: vec![TextSection {
                                        style: TextStyle {
                                            color: Color::rgb(1.0, 1.0, 1.0),
                                            font: asset_server
                                                .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                            font_size: 16.0,
                                        },
                                        value: license.text().to_string(),
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ));

                        parent.spawn((
                            Name::new(license.text().to_string()),
                            TextBundle {
                                text: Text {
                                    alignment: TextAlignment::Left,
                                    sections: vec![TextSection {
                                        style: TextStyle {
                                            color: Color::rgb(0.8, 0.8, 0.8),
                                            font: asset_server
                                                .load(ASSET_FONT_FIRA_SANS_BOLD.path()),
                                            font_size: 12.0,
                                        },
                                        value: license.name().to_string(),
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ));
                    });
            }
        });
}
