use bevy::ecs::resource::Resource;
use bevy::utils::default;
use cpal::BufferSize;
use kira::backend::cpal::CpalBackendSettings;
use kira::{AudioManagerSettings, DefaultBackend, track::MainTrackBuilder};

/// This resource is used to configure the audio backend at creation
///
/// It needs to be inserted before adding the [`AudioPlugin`](crate::AudioPlugin) and will be
/// consumed by it. Settings cannot be changed at run-time!
#[derive(Resource)]
pub struct AudioSettings {
    /// The maximum number of sounds that can be playing at a time.
    pub sound_capacity: usize,
    /// Cpal backend buffer size. Use `BufferSize::Fixed(2048)` to reduce audio
    /// stuttering caused by frame drops.
    pub buffer_size: BufferSize,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            sound_capacity: 128,
            buffer_size: BufferSize::Default,
        }
    }
}

impl From<AudioSettings> for AudioManagerSettings<DefaultBackend> {
    fn from(settings: AudioSettings) -> Self {
        AudioManagerSettings {
            main_track_builder: MainTrackBuilder::new().sound_capacity(settings.sound_capacity),
            backend_settings: CpalBackendSettings {
                buffer_size: settings.buffer_size,
                ..default()
            },
            ..default()
        }
    }
}
