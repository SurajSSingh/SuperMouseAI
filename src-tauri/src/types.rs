//! Data types and associated functions for those types.

use crate::mutter::Model;
use log::{debug, warn};
use mouce::common::MouseButton;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{collections::HashMap, path::PathBuf};
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
    pub(crate) sound_map: HashMap<String, PathBuf>,
}

impl InnerAppState {
    pub const fn new(model: Model, sound_map: HashMap<String, PathBuf>) -> Self {
        // Load model into memory by evaluating short silence
        // FIXME: Need to do this in another thread, otherwise UI freezes
        // let _ = model.transcribe_pcm_s16le(&[0.0; 20_000], false, false, None, None, None);
        Self {
            model: ModelHolder {
                default: model,
                custom: None,
            },
            sound_map,
        }
    }

    /// Get sound by the provided name or by prepending `default_` to the beginning.
    pub fn get_sound_path(&self, sound_name: &str) -> Option<&PathBuf> {
        debug!("Getting sound: {sound_name}");
        self.sound_map.get(sound_name).or_else(|| {
            warn!("No sound with name '{sound_name}', falling back to 'default_{sound_name}'");
            self.sound_map.get(&format!("default_{}", &sound_name))
        })
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

pub type AppState = std::sync::Mutex<InnerAppState>;

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
    #[allow(clippy::upper_case_acronyms)]
    SRT,
    #[allow(clippy::upper_case_acronyms)]
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

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Type)]
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
