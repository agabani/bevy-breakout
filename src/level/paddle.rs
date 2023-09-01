use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Paddle;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Paddle>>,
) {
    let mut controller = query.single_mut();

    if input.pressed(KeyCode::Left) {
        controller.translation = Some(Vec2::new(-1.0, 0.0));
    }

    if input.pressed(KeyCode::Right) {
        controller.translation = Some(Vec2::new(1.0, 0.0));
    }
}

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn((
        // metadata
        Name::new("Paddle"),
        Paddle,
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
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(120.0, 20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Paddle>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
