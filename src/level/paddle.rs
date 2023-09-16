use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    look_at::{LookAt, LookAtNormal},
    physics::MIN_Z,
};

#[derive(Component)]
pub(crate) struct Paddle;

#[derive(Component)]
pub(crate) struct PaddleLookAt;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn movement_keyboard(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut KinematicCharacterController, &mut Transform), With<Paddle>>,
) {
    let (mut controller, transform) = query.single_mut();

    if input.pressed(KeyCode::Left) {
        controller.translation = Some(Vec2::new(-5.0, -240.0 - transform.translation.y));
    }

    if input.pressed(KeyCode::Right) {
        controller.translation = Some(Vec2::new(5.0, -240.0 - transform.translation.y));
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn movement_touches(
    touches: Res<Touches>,
    mut query: Query<(&mut KinematicCharacterController, &mut Transform), With<Paddle>>,
    window: Query<&Window>,
) {
    let (mut controller, transform) = query.single_mut();
    let half_screen_width = window.single().resolution.width() / 2.0;

    for finger in touches.iter() {
        if finger.position().x - half_screen_width < transform.translation.x {
            controller.translation = Some(Vec2::new(-5.0, -240.0 - transform.translation.y));
        }

        if finger.position().x - half_screen_width > transform.translation.x {
            controller.translation = Some(Vec2::new(5.0, -240.0 - transform.translation.y));
        }
    }
}

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
        Collider::cuboid(0.5, 0.5),
        KinematicCharacterController::default(),
        RigidBody::KinematicPositionBased,
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
