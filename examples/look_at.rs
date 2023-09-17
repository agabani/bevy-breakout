use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_breakout::prelude::*;

#[derive(Component)]
pub(crate) struct Actor;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, LookAtPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, cursor_position);

    #[cfg(feature = "dev")]
    app.add_plugins(DevPlugin);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let id = commands
        .spawn((
            Name::new("Target"),
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.5, 0.5))),
                mesh: meshes.add(shape::Circle::default().into()).into(),
                transform: Transform {
                    scale: Vec3::new(30.0, 30.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    commands.spawn((
        Name::new("Actor"),
        Actor,
        LookAt {
            normal: LookAtNormal::Vec2(Vec2::Y),
            entity: id,
        },
        MaterialMesh2dBundle {
            material: materials.add(ColorMaterial::from(Color::rgb(1.0, 0.5, 0.5))),
            mesh: meshes.add(shape::Quad::default().into()).into(),
            transform: Transform {
                scale: Vec3::new(60.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -50.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn cursor_position(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut actor: Query<&mut Transform, With<Actor>>,
) {
    let mut actor = actor.single_mut();
    let (camera, camera_transform) = cameras.single();
    let window = windows.single();

    let position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    if let Some(position) = position {
        actor.translation = position.extend(0.0);
    }
}
