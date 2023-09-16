use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::math::brick_circular::BrickCircular;

use super::ball::Ball;

#[derive(Component)]
pub(crate) struct Brick;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn collision_ball(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    balls: Query<Entity, With<Ball>>,
    bricks: Query<Entity, With<Brick>>,
) {
    for &collision_event in &mut collision_events {
        if let CollisionEvent::Stopped(a, b, _) = collision_event {
            if let (Ok(_), Ok(brick)) = (balls.get(a), bricks.get(b)) {
                commands.entity(brick).despawn_recursive();
            }

            if let (Ok(_), Ok(brick)) = (balls.get(b), bricks.get(a)) {
                commands.entity(brick).despawn_recursive();
            }
        }
    }
}

pub(crate) fn setup(mut commands: Commands) {
    let spec = BrickCircular {
        columns: 11,
        rows: 5,
        radius_max: 200.0,
        radius_min: 50.0,
        offset_x: 0.0,
        offset_y: 0.0,
        padding_x: 8.0,
        padding_y: 4.0,
    };

    for row in 0..spec.rows {
        for column in 0..spec.columns {
            commands.spawn((
                // metadata
                Name::new(format!("Brick x:{column} y:{row}")),
                Brick,
                // physics
                ActiveEvents::COLLISION_EVENTS,
                Collider::cuboid(0.5, 0.5),
                GravityScale(0.0),
                RigidBody::Fixed,
                // sprite
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.8),
                        ..Default::default()
                    },
                    transform: spec.transform(column, row),
                    ..Default::default()
                },
            ));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
