use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub(crate) struct Ball;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // metadata
        Name::new("Ball"),
        Ball,
        CollidableBundle::new(Collider::ball(0.5)),
        // physics
        Ccd::enabled(),
        ColliderMassProperties::Mass(0.01),
        Damping {
            angular_damping: 0.0,
            linear_damping: 0.0,
        },
        GravityScale(0.0),
        RigidBody::Dynamic,
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        VelocitizedBundle::new(400.0, Vec2::Y),
        // mesh
        MaterialMesh2dBundle {
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.5, 0.5))),
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -220.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Ball>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
