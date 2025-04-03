import { BaseDirectory } from "@tauri-apps/api/path";
import type { WhisperModelInfo } from "./types.ts";

/**
 * The default model package with the app when installed
 */
export const DEFAULT_MODEL: WhisperModelInfo = {
  "name": "Tiny (High Compression, English-Only)",
  "relativePath": "ggml-tiny_q5_1.en.bin",
  "sha256": "c77c5766f1cef09b6b7d47f21b546cbddd4157886b3b5d6d4f709e91e66c7c2b",
  "approxSize": "32.2 MB",
};

/**
 * Directory to the downloaded models
 */
export const MODELS_DIR = "models";

/**
 * URL from which models can be downloaded from
 */
export const MODEL_BASE_URL =
  "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

/**
 * Basic option for base directory to be the local app data location
 */
export const BASE_LOCAL_APP_DIR = {
  baseDir: BaseDirectory.AppLocalData,
};

/**
 * A List of all GGML Whisper models hosted on Hugging Face.
 *
 * Please see <https://huggingface.co/ggerganov/whisper.cpp> for details
 */
export const WHISPER_GGML_MODELS: WhisperModelInfo[] = [
  {
    "name": "Tiny (Low Compression, English-Only)",
    "relativePath": "ggml-tiny.en-q8_0.bin",
    "sha256":
      "5bc2b3860aa151a4c6e7bb095e1fcce7cf12c7b020ca08dcec0c6d018bb7dd94",
    "approxSize": "43.6 MB",
  },
  {
    "name": "Tiny (Full size, English-Only)",
    "relativePath": "ggml-tiny.en.bin",
    "sha256":
      "921e4cf8686fdd993dcd081a5da5b6c365bfde1162e72b08d75ac75289920b1f",
    "approxSize": "77.7 MB",
  },
  {
    "name": "Large V3 Full",
    "relativePath": "ggml-large-v3.bin",
    "sha256":
      "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2",
    "approxSize": "3.1 GB",
  },
  //   TODO(@): Get the rest of the models
];
