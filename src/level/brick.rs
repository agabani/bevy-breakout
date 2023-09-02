use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Brick;

pub(crate) fn setup(mut commands: Commands) {
    for row in 1..=3 {
        for column in 1..=3 {
            commands.spawn(brick(Vec3::new(
                (120.0 + 40.0) * column as f32 - 300.0,
                (20.0 + 40.0) * row as f32 + 0.0,
                0.0,
            )));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn brick(translation: Vec3) -> impl Bundle {
    (
        // metadata
        Name::new("Brick"),
        Brick,
        // physics
        Collider::cuboid(0.5, 0.5),
        GravityScale(0.0),
        RigidBody::Dynamic,
        // sprite
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                ..Default::default()
            },
            transform: Transform {
                translation,
                scale: Vec3::new(120.0, 20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    )
}
