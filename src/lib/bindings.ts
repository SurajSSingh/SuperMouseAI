
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
/**
 * Take WAV audio data and transcribe it with application Whisper model.
 * 
 * Check [crate::mutter::Model::transcribe_audio] for details on arguments
 */
async transcribe(audioData: number[], options: TranscribeOptions | null) : Promise<Result<string, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe", { audioData, options }) };
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
async pasteText(text: string) : Promise<Result<null, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("paste_text", { text }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
/**
 * Process the text
 */
async processText(text: string, options: TextProcessOptions | null) : Promise<Result<string, string>> {
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
async transcribeWithPostProcess(audioData: number[], transcribeOptions: TranscribeOptions | null, processingOptions: TextProcessOptions | null) : Promise<Result<string, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("transcribe_with_post_process", { audioData, transcribeOptions, processingOptions }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/


export const events = __makeEvents__<{
modKeyEvent: ModKeyEvent,
mouseClickEvent: MouseClickEvent
}>({
modKeyEvent: "mod-key-event",
mouseClickEvent: "mouse-click-event"
})

/** user-defined constants **/



/** user-defined types **/

/**
 * Tauri event representing a modifier key press globally
 * 
 * ### Payload
 * 
 * [ModKeyPayload] : The modifer key that is pressed/released
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
 * [MouseButtonType] : Which button was pressed
 */
export type MouseClickEvent = MouseButtonType
/**
 * Ways a text can be decorated
 */
export type TextDecoration = "Bold" | "Italics" | "Underline" | "Strikethrough" | "Mark"
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
decorated_words: ([TextDecoration, string])[] | null }
/**
 * Options for the transcribing function.
 * 
 * All items are optional. Based on arguments for [crate::mutter::Model::transcribe_audio].
 */
export type TranscribeOptions = { translate: boolean | null; individual_word_timestamps: boolean | null; threads: number | null; initial_prompt: string | null; language: string | null; format: TranscriptionFormat | null }
/**
 * Format type for a transcription
 */
export type TranscriptionFormat = "Text" | "SRT" | "VTT"

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
