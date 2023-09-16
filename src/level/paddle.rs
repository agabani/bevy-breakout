use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    look_at::{LookAt, LookAtNormal},
    orbit_controller::{OrbitController, OrbitControllerBundle},
    physics::MIN_Z,
};

#[derive(Component)]
pub(crate) struct Paddle;

#[derive(Component)]
pub(crate) struct PaddleLookAt;

pub(crate) fn setup(mut commands: Commands) {
    let entity = commands
        .spawn((
            Name::new("Paddle Look At"),
            PaddleLookAt,
            Transform::from_xyz(0., 0.0, 0.0),
        ))
        .id();

    commands.spawn((
        // metadata
        Name::new("Paddle"),
        Paddle,
        LookAt {
            entity,
            normal: LookAtNormal::Vec2(Vec2::Y),
        },
        // physics
        OrbitControllerBundle::new(
            Collider::cuboid(0.5, 0.5),
            KinematicCharacterController::default(),
            OrbitController {
                altitude: 250.0,
                entity,
            },
        ),
        // sprite
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -240.0, 0.0),
                scale: Vec3::new(120.0, 20.0, MIN_Z),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(
    mut commands: Commands,
    paddles: Query<Entity, With<Paddle>>,
    paddle_look_ats: Query<Entity, With<PaddleLookAt>>,
) {
    if let Ok(entity) = paddles.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    if let Ok(entity) = paddle_look_ats.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
