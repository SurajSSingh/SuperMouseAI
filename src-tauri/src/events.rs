//! Module holding all events for Super Mouse AI, as well as all associated functions that either react to or emit events.

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

use crate::types::{ModKeyPayload, MouseButtonType};

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing mouse click globally
///
/// ### Payload
///
/// [MouseButtonType] : Which button was pressed
pub struct MouseClickEvent(MouseButtonType);

impl MouseClickEvent {
    pub fn with_payload(payload: MouseButtonType) -> Self {
        MouseClickEvent(payload)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing a modifier key press globally
///
/// ### Payload
///
/// [ModKeyPayload] : The modifer key that is pressed/released
pub struct ModKeyEvent(ModKeyPayload);

impl ModKeyEvent {
    pub fn with_payload(payload: ModKeyPayload) -> Self {
        ModKeyEvent(payload)
    }
}
