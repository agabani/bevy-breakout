use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::math::brick::Spec;

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
    let spec = Spec {
        columns: 8,
        rows: 6,
        border_bottom: 000.0,
        border_left: -290.0,
        border_right: 290.0,
        border_top: 290.0,
        padding_x: 8.0,
        padding_y: 8.0,
    };

    for row in 0..spec.rows {
        for column in 0..spec.columns {
            let layout = spec.transform(column, row);

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
                    transform: Transform {
                        translation: Vec3::new(layout.x, layout.y, 0.0),
                        scale: Vec3::new(layout.width, layout.height, 0.0),
                        ..Default::default()
                    },
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
