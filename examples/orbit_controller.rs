use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_breakout::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, PhysicsPlugin, OrbitControllerPlugin))
        .add_systems(Startup, setup);

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

    let entity = commands
        .spawn((
            Name::new("Focal Point"),
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
        Name::new("Controller"),
        OrbitControllerBundle::new(
            Collider::cuboid(0.5, 0.5),
            KinematicCharacterController::default(),
            OrbitController {
                altitude: 200.0,
                entity,
            },
        ),
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
