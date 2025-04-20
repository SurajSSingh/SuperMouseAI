//! Module holding all events for Super Mouse AI, as well as all associated functions that either react to or emit events.

use log::{debug, trace};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::{collect_events, Event, Events};
#[cfg(feature = "whisper-rs")]
use whisper_rs::SegmentCallbackData;

use crate::types::{ModKeyPayload, MouseButtonType};

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing mouse click globally
///
/// ### Payload
///
/// [`MouseButtonType`] : Which button was pressed
pub struct MouseClickEvent(MouseButtonType);

impl MouseClickEvent {
    pub fn with_payload(payload: MouseButtonType) -> Self {
        trace!("Mouse click EVENT with following payload: {payload:?}");
        Self(payload)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing a modifier key press globally
///
/// ### Payload
///
/// [`ModKeyPayload`] : The modifer key that is pressed/released
pub struct ModKeyEvent(ModKeyPayload);

impl ModKeyEvent {
    pub fn with_payload(payload: ModKeyPayload) -> Self {
        trace!("Modifier key EVENT with following payload: {payload:?}");
        Self(payload)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Tauri event representing mouse movement
///
/// ### Payload
///
/// x [i32] : Absolute X value of mosue (from 0 to `SCREEN_WIDTH`)
/// y [i32] : Absolute Y value of mouse (from 0 to `SCREEN_HEIGHT`)
pub struct MouseMoveEvent {
    x: i32,
    y: i32,
}

impl MouseMoveEvent {
    pub fn with_payload(x: i32, y: i32) -> Self {
        trace!("Mouse move EVENT with following payload: ({x}, {y})");
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Event representing the progress for the current transcription
///
/// ### Payload
///
/// [i32] : The integer percentage value (0-100)
/// _NOTE: Whisper Transcription progress is not very granular._
pub struct TranscriptionProgressEvent(i32);

impl TranscriptionProgressEvent {
    pub fn with_payload(progress: i32) -> Self {
        debug!("Transcription Progress: {progress}");
        Self(progress)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
/// Event representing the progress for the current transcription
///
/// ### Payload
///
/// - `is_lossy` [bool] : Whether the transcription data is lossy
/// - [`SegmentCallbackData`]: The current segment that was transcribed _(fields lifted directly into struct)_:
///     - `segment` [i32]: Segment index
///     - `start_timestamp` [i64]: Start of the segment
///     - `end_timestamp` [i64]: End of the segment
///     - `text` [`String`]: The text segment
pub struct TranscriptionSegmentEvent {
    pub is_lossy: bool,
    pub segment: i32,
    pub start_timestamp: f64,
    pub end_timestamp: f64,
    pub text: String,
}

impl TranscriptionSegmentEvent {
    #[allow(dead_code)]
    pub fn with_payload(
        is_lossy: bool,
        segment: i32,
        start_timestamp: f64,
        end_timestamp: f64,
        text: String,
    ) -> Self {
        debug!("New Transcription Segment: (lossy={is_lossy}, data=(i={segment},s={start_timestamp},e={end_timestamp},t.len={})", text.len());
        Self {
            is_lossy,
            segment,
            start_timestamp,
            end_timestamp,
            text,
        }
    }
}

/// Create a new lossy [`TranscriptionSegmentEvent`] with payload defined from given [`SegmentCallbackData`]
#[cfg(feature = "whisper-rs")]
#[allow(dead_code)]
pub fn new_lossy_transcript_segment_event(
    segment: SegmentCallbackData,
) -> TranscriptionSegmentEvent {
    #[allow(
        clippy::cast_precision_loss,
        reason = "Unlikely that user gives a audio file greater than 2^51 ms"
    )]
    TranscriptionSegmentEvent::with_payload(
        true,
        segment.segment,
        segment.start_timestamp as f64,
        segment.end_timestamp as f64,
        segment.text,
    )
}

/// Create a new [`TranscriptionSegmentEvent`] with payload defined from given [`SegmentCallbackData`]
#[cfg(feature = "whisper-rs")]
#[allow(dead_code)]
pub fn new_transcript_segment_event(segment: SegmentCallbackData) -> TranscriptionSegmentEvent {
    #[allow(
        clippy::cast_precision_loss,
        reason = "Unlikely that user gives a audio file greater than 2^51 ms"
    )]
    TranscriptionSegmentEvent::with_payload(
        false,
        segment.segment,
        segment.start_timestamp as f64,
        segment.end_timestamp as f64,
        segment.text,
    )
}

#[must_use]
pub fn get_collected_events() -> Events {
    collect_events![
        MouseClickEvent,
        ModKeyEvent,
        MouseMoveEvent,
        TranscriptionProgressEvent,
        TranscriptionSegmentEvent,
    ]
}
