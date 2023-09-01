use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Camera;

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera {}, Name::new("Camera")));
}

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn teardown(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
