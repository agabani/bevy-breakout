mod camera;
mod play_button;
mod quit_button;

use bevy::prelude::*;

use crate::GameState;

#[allow(clippy::module_name_repetitions)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::MainMenu),
            (camera::setup, main_menu_setup),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (camera::teardown, main_menu_teardown),
        )
        .add_systems(
            Update,
            (play_button::interaction, quit_button::interaction)
                .run_if(in_state(GameState::MainMenu)),
        );
    }
}

#[derive(Component)]
pub struct MainMenu {}

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
                    crate::plugins::button::ButtonBundle::new(),
                    play_button::PlayButton {},
                    Name::new("Play Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(crate::plugins::text::TextBundle::new(&asset_server, "Play"));
                });

            parent
                .spawn((
                    crate::plugins::button::ButtonBundle::new(),
                    quit_button::QuitButton {},
                    Name::new("Quit Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(crate::plugins::text::TextBundle::new(&asset_server, "Quit"));
                });
        });
}

#[allow(clippy::needless_pass_by_value)]
fn main_menu_teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}
