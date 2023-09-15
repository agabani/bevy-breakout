use bevy::prelude::*;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(crate) struct LookAt(Vec3);

impl LookAt {
    pub fn new(position: Vec3) -> Self {
        Self(position)
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub(crate) struct Looker;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn at(mut query: Query<(&mut Transform, &LookAt), With<Looker>>) {
    for (mut transform, target) in &mut query {
        transform.look_at(target.0, Vec3::Z);
    }
}
