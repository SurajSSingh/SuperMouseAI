
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
/**
 * Take WAV audio data and transcribe it with application Whisper model.
 * 
 * Check [crate::mutter::Model::transcribe_audio] for details on arguments
 */
async transcribe(audioData: number[], whisperOptions: TranscribeOptions | null, decodeOptions: AudioProcessingOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe", { audioData, whisperOptions, decodeOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Play the provided sound given its name that is stored in the app_state
 */
async playSound(soundName: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("play_sound", { soundName }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Paste text from clipboard
 */
async pasteText() : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("paste_text") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Process the text
 */
async processText(text: string, options: TextProcessOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("process_text", { text, options }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Run [transcribe] function then pass to [process_text] for post processing.
 */
async transcribeWithPostProcess(audioData: number[], transcribeOptions: TranscribeOptions | null, processingOptions: TextProcessOptions | null, decodeOptions: AudioProcessingOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe_with_post_process", { audioData, transcribeOptions, processingOptions, decodeOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Put window on top, can be overriden by optional parameter
 */
async setWindowTop(overrideValue: boolean | null) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_window_top", { overrideValue }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Paste text from clipboard
 */
async writeText(text: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("write_text", { text }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Update the custom model information
 */
async updateModel(path: string | null, useGpu: boolean | null) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("update_model", { path, useGpu }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Get information about the user's system.
 */
async getSystemInfo() : Promise<SystemInfo> {
    return await TAURI_INVOKE("get_system_info");
},
/**
 * Initialize/De-initialize the sentry plugin depending on the toggled value
 */
async sentryCrashReporterUpdate(enable: boolean) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("sentry_crash_reporter_update", { enable }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async startMicrophoneRecording() : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("start_microphone_recording") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async stopMicrophoneRecording(delay: number | null) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("stop_microphone_recording", { delay }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async transcribeCurrentData(transcribeOptions: TranscribeOptions | null, decodeOptions: AudioProcessingOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe_current_data", { transcribeOptions, decodeOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async transcribeCurrentThenProcess(transcribeOptions: TranscribeOptions | null, processingOptions: TextPostProcessing, decodeOptions: AudioProcessingOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe_current_then_process", { transcribeOptions, processingOptions, decodeOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async stopTranscribeAndProcessData(stopMicTime: number | null, transcribeOptions: TranscribeOptions | null, processingOptions: TextPostProcessing, decodeOptions: AudioProcessingOptions | null) : Promise<Result<[string, number], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("stop_transcribe_and_process_data", { stopMicTime, transcribeOptions, processingOptions, decodeOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async setInputDevice(index: number) : Promise<Result<boolean, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("set_input_device", { index }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getInputDevices() : Promise<Result<string[], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_input_devices") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getCurrentInputDevice() : Promise<Result<string | null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_current_input_device") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/


export const events = __makeEvents__<{
modKeyEvent: ModKeyEvent,
mouseClickEvent: MouseClickEvent,
transcriptionProgressEvent: TranscriptionProgressEvent,
transcriptionSegmentEvent: TranscriptionSegmentEvent
}>({
modKeyEvent: "mod-key-event",
mouseClickEvent: "mouse-click-event",
transcriptionProgressEvent: "transcription-progress-event",
transcriptionSegmentEvent: "transcription-segment-event"
})

/** user-defined constants **/



/** user-defined types **/

/**
 * Options for the processing audio for [`crate::mutter::decode_and_denoise`] function.
 * 
 * All items are optional.
 */
export type AudioProcessingOptions = { 
/**
 * Wheter to normalize audio, defaults to `false`
 */
normalize_result: boolean | null; 
/**
 * Wheter to denoise audio, defaults to `true`
 */
denoise_audio: boolean | null; 
/**
 * Value for low pass filter, this represents maximum frequency allowed, default is `3000`
 */
low_pass_value: number | null; 
/**
 * Value for high pass filter, this represents minimum frequency allowed, default is `200`
 */
high_pass_value: number | null }
/**
 * Tauri event representing a modifier key press globally
 * 
 * ### Payload
 * 
 * [`ModKeyPayload`] : The modifer key that is pressed/released
 */
export type ModKeyEvent = ModKeyPayload
/**
 * Information about modifier key event (pressed or released)
 */
export type ModKeyPayload = { key: string; is_pressed: boolean }
/**
 * Enum representing mouse button type
 * 
 * Main three are left, middle, and right
 */
export type MouseButtonType = "Left" | "Middle" | "Right"
/**
 * Tauri event representing mouse click globally
 * 
 * ### Payload
 * 
 * [`MouseButtonType`] : Which button was pressed
 */
export type MouseClickEvent = MouseButtonType
/**
 * Basic information about the current system
 */
export type SystemInfo = { 
/**
 * Total number of cores on this system
 */
cpu_core_count: number; 
/**
 * Total amount of system memory (RAM) in the system in GB
 */
total_memory_gb: number; 
/**
 * Total amount of graphic memory (VRAM) in the system in GB
 */
total_vram_gb: number }
/**
 * Ways a text can be decorated
 */
export type TextDecoration = "Bold" | "Italics" | "Underline" | "Strikethrough" | "Mark"
export type TextPostProcessing = "Skip" | "Default" | { Custom: TextProcessOptions }
/**
 * Options for the text post-processing function.
 * 
 * All items are optional.
 */
export type TextProcessOptions = { 
/**
 * Words that will be removed (or striken) from the string
 */
removed_words: string[] | null; 
/**
 * Words to modify in someway that does not change meaning of word,
 * but adds some decoration
 */
decorated_words: ([TextDecoration, string])[] | null; replace_inter_sentence_newlines: boolean | null }
/**
 * Options for the transcribing function.
 * 
 * All items are optional. Based on arguments for [`crate::mutter::Model::transcribe_audio`].
 */
export type TranscribeOptions = { translate: boolean | null; individual_word_timestamps: boolean | null; threads: number | null; initial_prompt: string | null; language: string | null; format: TranscriptionFormat | null; patience: number | null; include_callback: boolean | null }
/**
 * Format type for a transcription
 */
export type TranscriptionFormat = "Text" | "SRT" | "VTT"
/**
 * Tauri event representing mouse movement
 * 
 * ### Payload
 * 
 * x [i32] : Absolute X value of mosue (from 0 to `SCREEN_WIDTH`)
 * y [i32] : Absolute Y value of mouse (from 0 to `SCREEN_HEIGHT`)
 * Event representing the progress for the current transcription
 * 
 * ### Payload
 * 
 * [i32] : The integer percentage value (0-100)
 * _NOTE: Whisper Transcription progress is not very granular._
 */
export type TranscriptionProgressEvent = number
/**
 * Event representing the progress for the current transcription
 * 
 * ### Payload
 * 
 * - `is_lossy` [bool] : Whether the transcription data is lossy
 * - [`SegmentCallbackData`]: The current segment that was transcribed _(fields lifted directly into struct)_:
 * - `segment` [i32]: Segment index
 * - `start_timestamp` [i64]: Start of the segment
 * - `end_timestamp` [i64]: End of the segment
 * - `text` [`String`]: The text segment
 */
export type TranscriptionSegmentEvent = { is_lossy: boolean; segment: number; start_timestamp: number; end_timestamp: number; text: string }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
