use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};

#[allow(clippy::module_name_repetitions)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        app.add_event::<PlayEvent>()
            .add_systems(Update, (change_track, fade_volume));

        #[cfg(feature = "dev")]
        app.register_type::<BackgroundMusic>()
            .register_type::<FadeVolumeTo>()
            .register_type::<PlayEvent>();
    }
}

#[derive(Event)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct PlayEvent {
    pub path: bevy::asset::AssetPath<'static>,
}

impl PlayEvent {
    pub fn new(path: impl Into<bevy::asset::AssetPath<'static>>) -> Self {
        Self { path: path.into() }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct BackgroundMusic;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
struct FadeVolumeTo {
    value: f32,
}

#[allow(clippy::needless_pass_by_value)]
fn change_track(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PlayEvent>,
    query: Query<Entity, With<BackgroundMusic>>,
) {
    for event in &mut events {
        for entity in &query {
            commands.entity(entity).despawn_recursive();
        }

        commands.spawn((
            AudioBundle {
                source: asset_server.load(event.path.clone()),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    volume: Volume::Relative(VolumeLevel::new(0.0)),
                    ..default()
                },
            },
            BackgroundMusic,
            FadeVolumeTo { value: 0.5 },
        ));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn fade_volume(mut commands: Commands, query: Query<(Entity, &AudioSink, &FadeVolumeTo)>) {
    for (entity, sink, fade_volume_to) in query.iter() {
        let current = sink.volume();
        let target = fade_volume_to.value;

        #[allow(clippy::float_cmp)]
        if current == target {
            commands.entity(entity).remove::<FadeVolumeTo>();
        } else {
            let diff = target - current;
            let delta = diff.min(0.01);
            let volume = current + delta;
            sink.set_volume(volume);
        }
    }
}
