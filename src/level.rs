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
            .add_systems(OnExit(GameState::Level), paddle_teardown);
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
