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
  modelSize: "tiny" | "base" | "small" | "medium" | "large";
  version?: "v1" | "v2" | "v3" | "v3 turbo";
  relativePath: string;
  sha256: string;
  approxSize: number; // bytes
  quantizeType: "full" | "q8" | "q5";
  isEnglishOnly: boolean;
  isSuperceded: boolean | string;
  recommendedVramForGPU?: number; // bytes
  recommendedRamForCPU?: number; // bytes
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

/**
 * Options for the [`LargestModelFinderOption`]
 */
export type LargestModelFinderOption = {
  preferEnglishOnly: boolean;
  usesCPU: boolean;
  compressionAllowed: "all" | "low" | "full-only";
  minRAMRatio: number;
  maxRAMRatio: number;
  minVRAMRatio: number;
  maxVRAMRatio: number;
  /**
   * Priority of models, higher priority number comes earlier in sort than lower number
   */
  sizePriority: {
    largeTurbo: number;
    small: number;
    large: number;
    base: number;
    medium: number;
    tiny: number;
  };
};
