//! Data types and associated functions for those types.

use crate::mutter::Model;
use log::{debug, warn};
use mouce::common::MouseButton;
use rodio::{
    cpal::{default_host, traits::HostTrait, Host, StreamConfig},
    Device,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::async_runtime::Sender;
use whisper_rs::{WhisperContextParameters, WhisperError};

/// A struct to hold both the default and custom model together, enabling for easy switching
pub struct ModelHolder {
    default: Model,
    custom: Option<(Model, String)>,
}

/// "Global" state for the application.
///
/// This holds all data that is expected to
/// persist throughout the app's runtime.
pub struct InnerAppState {
    pub(crate) model: ModelHolder,
}

impl InnerAppState {
    pub const fn new(model: Model) -> Self {
        // Load model into memory by evaluating short silence
        // FIXME: Need to do this in another thread, otherwise UI freezes
        // let _ = model.transcribe_pcm_s16le(&[0.0; 20_000], false, false, None, None, None);
        Self {
            model: ModelHolder {
                default: model,
                custom: None,
            },
        }
    }

    /// Replace the custom model being used
    pub fn replace_custom_model(
        &mut self,
        path: String,
        use_gpu: bool,
    ) -> Result<(), WhisperError> {
        let mut params = WhisperContextParameters::new();
        params.use_gpu(use_gpu);
        let new_model = Model::new_with_params(&path, params)?;
        // _ will drop old model
        let _ = self.model.custom.replace((new_model, path));
        Ok(())
    }

    /// Removes a custom model
    pub fn remove_custom_model(&mut self) {
        // _ will drop old model
        let _ = self.model.custom.take();
    }

    pub fn get_model(&self) -> &Model {
        self.model
            .custom
            .as_ref()
            .map_or(&self.model.default, |holder| &holder.0)
    }

    pub fn get_model_info(&self) -> String {
        match &self.model {
            ModelHolder { custom: None, .. } => "Default Model".to_string(),
            ModelHolder {
                custom: Some((_, path)),
                ..
            } => {
                format!("Custom Model at: {path}")
            }
        }
    }
}

pub type AppState = Mutex<InnerAppState>;

#[derive(Debug, Clone, Default)]
/// A struct representing a sound bank
pub struct InnerSoundMapState(HashMap<String, PathBuf>);

impl InnerSoundMapState {
    pub const fn with_map(map: HashMap<String, PathBuf>) -> Self {
        Self(map)
    }

    /// Get sound by the provided name or by prepending `default_` to the beginning.
    pub fn get_sound_path(&self, sound_name: &str) -> Option<&PathBuf> {
        debug!("Getting sound: {sound_name}");
        self.0.get(sound_name).or_else(|| {
            warn!("No sound with name '{sound_name}', falling back to 'default_{sound_name}'");
            self.0.get(&format!("default_{}", &sound_name))
        })
    }
}

pub type SoundMapState = Mutex<InnerSoundMapState>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
/// Current state of the microphone
#[non_exhaustive]
pub enum RecordingState {
    Stopped,
    Recording,
}

/// State of the microphone
pub struct InnerMicrophoneState {
    pub host: Host,
    pub device: Option<Device>,
    pub stream_sender: Option<Sender<()>>,
}

impl InnerMicrophoneState {
    pub fn new() -> Self {
        Self::with_host(default_host())
    }

    pub fn with_host(host: Host) -> Self {
        let device = host.default_input_device();
        Self {
            host,
            device,
            stream_sender: None,
        }
    }

    /// Check if currently being recorded
    pub const fn is_recording(&self) -> bool {
        self.stream_sender.is_some()
    }

    // TODO: Add switching devices

    // TODO: Encapsulate data to allow independent access
}

pub type MicrophoneState = Mutex<InnerMicrophoneState>;

#[derive(Debug, Clone, Default)]
/// Data recorded from user's microphone
pub struct InnerMicrophoneData(pub Vec<f32>, pub u16, pub u32);

impl InnerMicrophoneData {
    pub const fn new() -> Self {
        Self(Vec::new(), 1, 48_000)
    }

    /// Replace microphone data with one that follows a specific stream configuration
    pub fn replace_with_config(&mut self, audio_config: StreamConfig) {
        self.0.clear();
        self.1 = audio_config.channels;
        self.2 = audio_config.sample_rate.0;
    }

    /// Update based on stream configuration
    pub fn update_from_config(&mut self, audio_config: &StreamConfig) {
        self.1 = audio_config.channels;
        self.2 = audio_config.sample_rate.0;
    }
}
pub type MicrophoneDataState = Mutex<InnerMicrophoneData>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
/// Enum representing mouse button type
///
/// Main three are left, middle, and right
#[non_exhaustive]
pub enum MouseButtonType {
    Left,
    Middle,
    Right,
}

impl From<&MouseButton> for MouseButtonType {
    fn from(mb: &MouseButton) -> Self {
        match mb {
            MouseButton::Left => Self::Left,
            MouseButton::Middle => Self::Middle,
            MouseButton::Right => Self::Right,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Type)]
/// Information about modifier key event (pressed or released)
pub struct ModKeyPayload {
    key: String,
    is_pressed: bool,
}

impl ModKeyPayload {
    /// New Modifier Key Event
    pub const fn new(key: String, is_pressed: bool) -> Self {
        Self { key, is_pressed }
    }

    /// New pressed event for given key
    pub const fn pressed(key: String) -> Self {
        Self::new(key, true)
    }

    /// New release event for given key
    pub const fn released(key: String) -> Self {
        Self::new(key, false)
    }
}

/// Given a key, check if it matches one of the (specific) modifier keys.
///
/// The main modifiers are: Alt, Control, Meta, Option, and Shift (both left and right).
pub const fn is_modkey(key: device_query::Keycode) -> bool {
    use device_query::Keycode as K;
    matches!(
        key,
        K::Command
            | K::LAlt
            | K::LControl
            | K::LMeta
            | K::LOption
            | K::LShift
            | K::RAlt
            | K::RControl
            | K::RMeta
            | K::ROption
            | K::RShift
    )
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Type)]
#[non_exhaustive]
/// Format type for a transcription
pub enum TranscriptionFormat {
    #[default]
    Text,
    #[allow(clippy::upper_case_acronyms, reason = "Proper name of format type")]
    SRT,
    #[allow(clippy::upper_case_acronyms, reason = "Proper name of format type")]
    VTT,
}

impl TranscriptionFormat {
    /// Convert a given transcript to its string form based on the current format type.
    pub fn convert_transcript(self, transcript: &crate::transcript::Transcript) -> String {
        match self {
            Self::Text => transcript.as_text(),
            Self::SRT => transcript.as_srt(),
            Self::VTT => transcript.as_vtt(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the transcribing function.
///
/// All items are optional. Based on arguments for [`crate::mutter::Model::transcribe_audio`].
pub struct TranscribeOptions {
    pub translate: Option<bool>,
    pub individual_word_timestamps: Option<bool>,
    pub threads: Option<u16>,
    pub initial_prompt: Option<String>,
    pub language: Option<String>,
    pub format: Option<TranscriptionFormat>,
    pub patience: Option<f32>,
    pub include_callback: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type)]
#[non_exhaustive]
/// Ways a text can be decorated
pub enum TextDecoration {
    Bold,
    Italics,
    Underline,
    Strikethrough,
    Mark,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the text post-processing function.
///
/// All items are optional.
pub struct TextProcessOptions {
    /// Words that will be removed (or striken) from the string
    pub removed_words: Option<Vec<String>>,
    /// Words to modify in someway that does not change meaning of word,
    /// but adds some decoration
    pub decorated_words: Option<Vec<(TextDecoration, String)>>,
    pub replace_inter_sentence_newlines: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Type)]
/// Basic information about the current system
pub struct SystemInfo {
    /// Total number of cores on this system
    pub cpu_core_count: f64,
    /// Total amount of system memory (RAM) in the system in GB
    pub total_memory_gb: f64,
    /// Total amount of graphic memory (VRAM) in the system in GB
    pub total_vram_gb: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the processing audio for [`crate::mutter::decode_and_denoise`] function.
///
/// All items are optional.
pub struct AudioProcessingOptions {
    /// Wheter to normalize audio, defaults to `false`
    pub normalize_result: Option<bool>,
    /// Wheter to denoise audio, defaults to `true`
    pub denoise_audio: Option<bool>,
    /// Value for low pass filter, this represents maximum frequency allowed, default is `3000`
    pub low_pass_value: Option<u32>,
    /// Value for high pass filter, this represents minimum frequency allowed, default is `200`
    pub high_pass_value: Option<u32>,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Type)]
///
///
///
pub enum TextPostProcessing {
    Skip,
    #[default]
    Default,
    Custom(TextProcessOptions),
}

impl TextPostProcessing {
    // /// Create a custom post-processing text with given [`TextProcessOptions`]
    // pub fn with_options(options: TextProcessOptions) -> Self {
    //     Self::Custom(options)
    // }

    /// Transform into options, None if skipped, using [`TextProcessOptions::default()`] for [`Self::Default`] and provided options for [`Self::Custom`]
    pub fn into_options(self) -> Option<TextProcessOptions> {
        match self {
            Self::Skip => None,
            Self::Default => Some(TextProcessOptions::default()),
            Self::Custom(text_process_options) => Some(text_process_options),
        }
    }
}
