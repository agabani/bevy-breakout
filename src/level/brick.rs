use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    for row in 0..3 {
        for column in 0..4 {
            commands.spawn(brick(column, row));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn brick(column: u32, row: u32) -> impl Bundle {
    (
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
            transform: brick_transform(column, row),
            ..Default::default()
        },
    )
}

#[allow(clippy::cast_precision_loss)]
fn brick_transform(column: u32, row: u32) -> Transform {
    let columns = 4;
    let rows = 3;

    let bottom = 100.0;
    let top = 200.0;
    let space_height = top - bottom;

    let left = -200.0;
    let right = 300.0;
    let space_width = right - left;

    let brick_height = space_height / rows as f32;
    let brick_width = space_width / columns as f32;

    let horizontal_gap = 8.0;
    let vertical_gap = 8.0;

    Transform {
        translation: Vec3::new(
            left + horizontal_gap / 2.0 + brick_width * column as f32,
            bottom + vertical_gap / 2.0 + brick_height * row as f32,
            0.0,
        ),
        scale: Vec3::new(
            brick_width - horizontal_gap,
            brick_height - vertical_gap,
            0.0,
        ),
        ..Default::default()
    }
}
