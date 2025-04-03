import type { WhisperModelInfo } from "./types.ts";

/**
 * A List of all GGML Whisper models hosted on Hugging Face.
 *
 * Please see <https://huggingface.co/ggerganov/whisper.cpp> for details
 */
export const WHISPER_GGML_MODELS: WhisperModelInfo[] = [
  //   {
  //     "name": "Tiny (High Compression, English-Only)",
  //     "relativePath": "ggml-tiny_q5_1.en.bin",
  //     "sha256":
  //       "c77c5766f1cef09b6b7d47f21b546cbddd4157886b3b5d6d4f709e91e66c7c2b",
  //     "approxSize": "32.2 MB",
  //   },
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
  //   TODO(@): Get the rest of the models
];
