use bevy::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct LookAtPlugin;

impl Plugin for LookAtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, look_at);

        #[cfg(feature = "dev")]
        app.register_type::<LookAt>();
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct LookAt {
    pub entity: Entity,
    pub normal: LookAtNormal,
}

#[allow(clippy::module_name_repetitions)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub enum LookAtNormal {
    Vec2(Vec2),
}

#[allow(clippy::needless_pass_by_value)]
fn look_at(
    mut transforms: Query<&mut Transform>,
    entities: Query<(Entity, &LookAt), With<Transform>>,
) {
    for (entity, look_at) in &entities {
        let target = *transforms
            .get(look_at.entity)
            .expect("TODO: target no longer exists");

        let mut source = transforms
            .get_mut(entity)
            .expect("TODO: source no longer exists");

        source.rotation = match look_at.normal {
            LookAtNormal::Vec2(normal) => {
                let direction = target.translation.truncate() - source.translation.truncate();
                let angle = direction.angle_between(normal);
                Quat::from_rotation_z(-angle)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use bevy::prelude::*;

    use crate::prelude::*;

    #[test]
    fn look_at() {
        // Arrange
        let mut app = App::new();

        app.add_plugins((BevyMinimalPlugin, LookAtPlugin));

        let entity = app.world.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();

        let test_cases = vec![
            ("right", Transform::from_xyz(1.0, 0.0, 0.0), PI / 2.0),
            (
                "top right",
                Transform::from_xyz(1.0, 1.0, 0.0),
                PI / 4.0 * 3.0,
            ),
            ("top", Transform::from_xyz(0.0, 1.0, 0.0), PI),
            (
                "top left",
                Transform::from_xyz(-1.0, 1.0, 0.0),
                PI / 4.0 * 3.0,
            ),
            ("left", Transform::from_xyz(-1.0, 0.0, 0.0), PI / 2.0),
            (
                "bottom left",
                Transform::from_xyz(-1.0, -1.0, 0.0),
                PI / 4.0,
            ),
            ("bottom", Transform::from_xyz(0.0, -1.0, 0.0), 0.0),
            (
                "bottom right",
                Transform::from_xyz(1.0, -1.0, 0.0),
                PI / 4.0,
            ),
        ]
        .into_iter()
        .map(|(name, transform, quat)| {
            let id = app
                .world
                .spawn((
                    transform,
                    LookAt {
                        entity,
                        normal: LookAtNormal::Vec2(Vec2::Y),
                    },
                ))
                .id();
            (name, id, quat)
        })
        .collect::<Vec<_>>();

        // Act
        app.update();

        // Assert
        for (name, entity, want) in test_cases {
            let transform = app.world.get::<Transform>(entity).unwrap();

            let got = transform.rotation.angle_between(Quat::from_rotation_z(0.0));

            assert!(
                (want - got).abs() < 0.00001,
                "{} wanted {} got {}",
                name,
                want,
                got
            );
        }
    }
}
