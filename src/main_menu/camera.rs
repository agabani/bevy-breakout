use bevy::prelude::*;

#[derive(Component)]
pub struct Camera {}

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Camera {}, Name::new("Camera")));
}

#[allow(clippy::needless_pass_by_value)]
pub fn teardown(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}
