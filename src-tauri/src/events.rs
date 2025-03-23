//! Module holding all events for Super Mouse AI, as well as all associated functions that either react to or emit events.

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::{collect_events, Event, Events};

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

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing mouse movement
///
/// ### Payload
///
/// x [i32] : Absolute X value of mosue (from 0 to SCREEN_WIDTH)
/// y [i32] : Absolute Y value of mouse (from 0 to SCREEN_HEIGHT)
pub struct MouseMoveEvent {
    x: i32,
    y: i32,
}

impl MouseMoveEvent {
    pub fn with_payload(x: &i32, y: &i32) -> Self {
        Self { x: *x, y: *y }
    }
}

pub fn get_collected_events() -> Events {
    collect_events![MouseClickEvent, ModKeyEvent, MouseMoveEvent]
}
