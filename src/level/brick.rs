use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Brick;

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn((
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
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(120.0, 20.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Brick>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
