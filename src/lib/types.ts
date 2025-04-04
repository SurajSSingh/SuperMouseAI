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
  modelSize?: "Tiny" | "Base" | "Small" | "Medium" | "Large";
  version?: "V1" | "V2" | "V3" | "V3 Turbo";
  relativePath: string;
  sha256: string;
  approxSize: string | { size: number; unit: "KB" | "MB" | "GB" };
  quantizeType: "full" | "q8" | "q5";
  isEnglishOnly?: boolean;
  isSuperceded?: boolean | string;
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
