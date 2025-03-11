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
