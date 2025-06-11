use crate::prelude::*;
use bevy::audio::AudioPlugin;
use bevy::math::ops::sin;
use rand::rng;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(AudioSettings, MusicAudio, UiAudio)>();
    app.add_plugins(AudioPlugin::default());
}

/// Audio settings.
#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Configure for AudioSettings {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();

        app.add_systems(
            Update,
            apply_audio_settings
                .run_if(resource_changed::<Self>)
                .in_set(AppSystems::Update),
        );
    }
}

impl AudioSettings {
    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.music_volume)
    }

    pub fn ui_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.ui_volume)
    }
}

fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    music_audio_query: Query<Entity, With<MusicAudio>>,
    ui_audio_query: Query<Entity, With<UiAudio>>,
    mut volume_query: Query<(Option<&mut PlaybackSettings>, Option<&mut AudioSink>)>,
) {
    // Apply music volume
    let volume = audio_settings.music_volume();
    for entity in &music_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply UI volume
    let volume = audio_settings.ui_volume();
    for entity in &ui_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));
        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }
}

/// A component to indicate that an audio source is music.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct MusicAudio;

impl Configure for MusicAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn music_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        // Name::new("Music Audio"),
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_settings.music_volume()),
        MusicAudio,
    )
}

/// A component to indicate that an audio source is music.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct UiAudio;

impl Configure for UiAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

pub fn ui_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        // Name::new("UI Audio"),
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN
            .with_volume(audio_settings.ui_volume())
            .with_speed(rng().random_range(0.9..1.5)),
        UiAudio,
    )
}
