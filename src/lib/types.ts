/**
 * States an audio recorder can be in.
 * 
 * Implemented as a tri-state.
 */
export type RecordingStates = "stopped" | "recording" | "processing";

/**
 * The kind type for theme value
 */
export type ThemeKind = "system" | "light" | "dark";