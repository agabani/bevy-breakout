use bevy::prelude::*;

use crate::GameState;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            // camera
            .add_systems(OnEnter(GameState::Level), camera_setup)
            .add_systems(OnExit(GameState::Level), camera_teardown)
            // paddle
            .add_systems(OnEnter(GameState::Level), paddle_setup)
            .add_systems(OnExit(GameState::Level), paddle_teardown)
            .add_systems(Update, paddle_movement.run_if(in_state(GameState::Level)))
            // menu
            .add_systems(Update, menu.run_if(in_state(GameState::Level)));
    }
}

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Paddle;

fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera {}, Name::new("Camera")));
}

#[allow(clippy::needless_pass_by_value)]
fn camera_teardown(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn paddle_setup(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(120.0, 20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle,
        Name::new("Paddle"),
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn paddle_teardown(mut commands: Commands, query: Query<Entity, With<Paddle>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::needless_pass_by_value)]
fn paddle_movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Paddle>>) {
    let mut transform = query.single_mut();

    if input.pressed(KeyCode::Left) {
        transform.translation.x -= 1.0;
    }

    if input.pressed(KeyCode::Right) {
        transform.translation.x += 1.0;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn menu(mut next_state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
