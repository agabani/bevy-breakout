mod camera;
mod play_button;
mod quit_button;
mod settings_button;

use bevy::prelude::*;

use crate::scenes::SceneState;

#[allow(clippy::module_name_repetitions)]
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::MainMenu), (camera::setup, setup))
            .add_systems(OnExit(SceneState::MainMenu), (camera::teardown, teardown))
            .add_systems(
                Update,
                (
                    play_button::interaction,
                    quit_button::interaction,
                    settings_button::interaction,
                )
                    .run_if(in_state(SceneState::MainMenu)),
            );

        #[cfg(feature = "dev")]
        app.register_type::<MainMenu>()
            .register_type::<play_button::PlayButton>()
            .register_type::<quit_button::QuitButton>()
            .register_type::<settings_button::SettingsButton>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct MainMenu;

#[allow(clippy::needless_pass_by_value)]
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MainMenu,
            Name::new("Main Menu"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    crate::components::button::ButtonBundle::new(),
                    play_button::PlayButton,
                    Name::new("Play Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(crate::components::text::TextBundle::new(
                        &asset_server,
                        "Play",
                    ));
                });

            parent
                .spawn((
                    crate::components::button::ButtonBundle::new(),
                    settings_button::SettingsButton,
                    Name::new("Settings Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(crate::components::text::TextBundle::new(
                        &asset_server,
                        "Settings",
                    ));
                });

            parent
                .spawn((
                    crate::components::button::ButtonBundle::new(),
                    quit_button::QuitButton,
                    Name::new("Quit Button"),
                ))
                .with_children(|parent| {
                    parent.spawn(crate::components::text::TextBundle::new(
                        &asset_server,
                        "Quit",
                    ));
                });
        });
}

#[allow(clippy::needless_pass_by_value)]
pub fn teardown(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
