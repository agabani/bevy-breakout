use bevy::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub(crate) struct Scoreboard;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Scoreboard,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::rgb(1.0, 1.0, 1.0),
            }),
        ]),
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Scoreboard>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn update(resource: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    for mut text in &mut query {
        text.sections[1].value = resource.value().to_string();
    }
}
