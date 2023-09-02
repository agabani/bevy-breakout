use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Wall;

pub(crate) fn setup(mut commands: Commands) {
    // top
    commands.spawn(wall(Transform {
        scale: Vec3::new(600.0, 20.0, 0.0),
        translation: Vec3::new(0.0, 300.0, 0.0),
        ..Default::default()
    }));
    // right
    commands.spawn(wall(Transform {
        scale: Vec3::new(20.0, 600.0, 0.0),
        translation: Vec3::new(300.0, 0.0, 0.0),
        ..Default::default()
    }));
    // bottom
    commands.spawn(wall(Transform {
        scale: Vec3::new(600.0, 20.0, 0.0),
        translation: Vec3::new(0.0, -300.0, 0.0),
        ..Default::default()
    }));
    // left
    commands.spawn(wall(Transform {
        scale: Vec3::new(20.0, 600.0, 0.0),
        translation: Vec3::new(-300.0, 0.0, 0.0),
        ..Default::default()
    }));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Wall>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn wall(transform: Transform) -> impl Bundle {
    (
        // metadata
        Name::new("Wall"),
        Wall,
        // physics
        Collider::cuboid(0.5, 0.5),
        GravityScale(0.0),
        RigidBody::Fixed,
        // sprite
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                ..Default::default()
            },
            transform,
            ..Default::default()
        },
    )
}
