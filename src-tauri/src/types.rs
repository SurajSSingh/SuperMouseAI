//! Data types and associated functions for those types.

use crate::mutter::Model;
use mouce::common::MouseButton;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{collections::HashMap, path::PathBuf};

/// "Global" state for the application.
///
/// This holds all data that is expected to
/// persist throughout the app's runtime.
pub struct AppState {
    pub(crate) model: Model,
    pub(crate) sound_map: HashMap<String, PathBuf>,
}

impl AppState {
    pub fn new(model: Model, sound_map: HashMap<String, PathBuf>) -> Self {
        // Load model into memory by evaluating short silence
        let _ = model.transcribe_pcm_s16le(&[0.0; 20_000], false, false, None, None, None);
        AppState { model, sound_map }
    }

    /// Get sound by the provided name or by prepending `default_` to the beginning.
    pub fn get_sound_path(&self, sound_name: &str) -> Option<&PathBuf> {
        self.sound_map
            .get(sound_name)
            .or_else(|| self.sound_map.get(&format!("default_{}", &sound_name)))
    }
}

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
            MouseButton::Left => MouseButtonType::Left,
            MouseButton::Middle => MouseButtonType::Middle,
            MouseButton::Right => MouseButtonType::Right,
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
    pub fn new(key: String, is_pressed: bool) -> Self {
        Self { key, is_pressed }
    }

    /// New pressed event for given key
    pub fn pressed(key: String) -> Self {
        Self::new(key, true)
    }

    /// New release event for given key
    pub fn released(key: String) -> Self {
        Self::new(key, false)
    }
}

/// Given a key, check if it matches one of the (specific) modifier keys.
///
/// The main modifiers are: Alt, Control, Meta, Option, and Shift (both left and right).
pub fn is_modkey(key: &device_query::Keycode) -> bool {
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
    SRT,
    VTT,
}

impl TranscriptionFormat {
    /// Convert a given transcript to its string form based on the current format type.
    pub fn convert_transcript(&self, transcript: crate::transcript::Transcript) -> String {
        match self {
            TranscriptionFormat::Text => transcript.as_text(),
            TranscriptionFormat::SRT => transcript.as_srt(),
            TranscriptionFormat::VTT => transcript.as_vtt(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Type)]
/// Options for the transcribing function.
///
/// All items are optional. Based on arguments for [crate::mutter::Model::transcribe_audio].
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

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize, Type)]
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
