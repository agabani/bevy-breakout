use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::math::brick_circular::BrickCircular;
use crate::prelude::*;

#[derive(Component)]
pub(crate) struct Brick;

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
                CollidableBundle::new(Collider::cuboid(0.5, 0.5)),
                DestructibleBundle::new(),
                ScorableBundle::new(),
                // physics
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
