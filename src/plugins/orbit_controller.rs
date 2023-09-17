use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct OrbitControllerPlugin;

impl Plugin for OrbitControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (keyboard, touch));

        #[cfg(feature = "dev")]
        app.register_type::<OrbitController>();
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct OrbitController {
    pub altitude: f32,
    pub entity: Entity,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Bundle)]
pub struct OrbitControllerBundle {
    collider: Collider,
    kinematic_character_controller: KinematicCharacterController,
    orbit_controller: OrbitController,
    rigid_body: RigidBody,
}

impl OrbitControllerBundle {
    #[must_use]
    pub fn new(
        collider: Collider,
        kinematic_character_controller: KinematicCharacterController,
        orbit_controller: OrbitController,
    ) -> Self {
        Self {
            collider,
            kinematic_character_controller,
            orbit_controller,
            rigid_body: RigidBody::KinematicPositionBased,
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn keyboard(
    input: Res<Input<KeyCode>>,
    query: Query<(Entity, &mut KinematicCharacterController, &OrbitController)>,
    transforms: Query<&Transform>,
) {
    translation(
        input.pressed(KeyCode::Left),
        input.pressed(KeyCode::Right),
        query,
        transforms,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn touch(
    touches: Res<Touches>,
    query: Query<(Entity, &mut KinematicCharacterController, &OrbitController)>,
    transforms: Query<&Transform>,
    window: Query<&Window>,
) {
    let half_screen_width = window.single().resolution.width() / 2.0;

    let mut left = false;
    let mut right = false;

    for touch in touches.iter() {
        if touch.position().x - half_screen_width < 0.0 {
            left = true;
        }
        if touch.position().x - half_screen_width > 0.0 {
            right = true;
        }
    }

    translation(left, right, query, transforms);
}

#[allow(clippy::needless_pass_by_value)]
fn translation(
    left: bool,
    right: bool,
    mut query: Query<(Entity, &mut KinematicCharacterController, &OrbitController)>,
    transforms: Query<&Transform>,
) {
    for (entity, mut controller, orbit) in &mut query {
        let source = transforms
            .get(entity)
            .expect("TODO: source no longer exists");

        let center = transforms
            .get(orbit.entity)
            .expect("TODO: center no longer exists");

        let source_direction = center.translation.truncate() - source.translation.truncate();
        let source_angle = source_direction.angle_between(Vec2::Y);

        let mut rotate_by = 0.0;

        if left {
            rotate_by += PI / 64.0;
        }
        if right {
            rotate_by -= PI / 64.0;
        }

        let target_angle = source_angle + rotate_by;
        let target_direction = Vec2::from_angle(-target_angle).rotate(Vec2::NEG_Y);

        let target = target_direction * orbit.altitude + center.translation.truncate();

        let translation = (target - source.translation.truncate()).clamp_length_max(24.0);

        if translation.length() > 0.01 {
            controller.translation = Some(translation);
        }
    }
}
