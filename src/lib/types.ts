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

/**
 * Options that may be passed to the confirmationAction function NotificationSystem
 */
export type ConfirmActionType = Partial<{
  sound: string;
  confirmButtonStyle: string;
  cancelButtonStyle: string;
  mustRetry: boolean | number;
}>;

/**
 * Whisper Model information
 */
export type WhisperModelInfo = {
  name: string;
  relativePath: string;
  sha256: string;
  approxSize: string;
  quantizeType: "full" | "q8" | "q5";
  isEnglishOnly?: boolean;
  isSuperceded?: boolean;
};

/**
 * Information related to a transcription
 */
export type TranscriptionInfo = {
  text: string;
  model?: string;
  onGPU?: boolean;
  processingTime?: number;
};
