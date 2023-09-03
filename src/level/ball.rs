use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

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
        // physics
        Collider::cuboid(0.5, 0.5),
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
        Velocity {
            linvel: Vect::new(0.0, 1.0).normalize() * 400.0,
            ..Default::default()
        },
        Sleeping {
            sleeping: false,
            ..Default::default()
        },
        // mesh
        MaterialMesh2dBundle {
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.5, 0.5))),
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -100.0, 0.0),
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
