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
  quantizeType: "q5",
  isEnglishOnly: true,
  isSuperceded: false,
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
    "name": "Tiny (Medium Compression, English-Only)",
    "relativePath": "ggml-tiny.en-q8_0.bin",
    "sha256":
      "5bc2b3860aa151a4c6e7bb095e1fcce7cf12c7b020ca08dcec0c6d018bb7dd94",
    "approxSize": "43.6 MB",
    quantizeType: "q8",
    isEnglishOnly: true,
  },
  {
    "name": "Tiny (Medium Compression)",
    "relativePath": "ggml-tiny-q8_0.bin",
    "sha256":
      "c2085835d3f50733e2ff6e4b41ae8a2b8d8110461e18821b09a15c40c42d1cca",
    "approxSize": "43.5 MB",
    quantizeType: "q8",
  },
  {
    "name": "Tiny (Full size, English-Only)",
    "relativePath": "ggml-tiny.en.bin",
    "sha256":
      "921e4cf8686fdd993dcd081a5da5b6c365bfde1162e72b08d75ac75289920b1f",
    "approxSize": "77.7 MB",
    quantizeType: "full",
    isEnglishOnly: true,
  },
  {
    "name": "Tiny (Full)",
    "relativePath": "ggml-tiny.bin",
    "sha256":
      "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
    "approxSize": "77.7 MB",
    quantizeType: "full",
  },
  {
    "name": "Base Full (English-Only)",
    "relativePath": "ggml-base.en.bin",
    "sha256":
      "a03779c86df3323075f5e796cb2ce5029f00ec8869eee3fdfb897afe36c6d002",
    "approxSize": "148 MB",
    quantizeType: "full",
    isEnglishOnly: true,
  },
  {
    "name": "Base Full",
    "relativePath": "ggml-base.bin",
    "sha256":
      "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe",
    "approxSize": "148 MB",
    quantizeType: "full",
  },
  {
    "name": "Small Full (English-Only)",
    "relativePath": "ggml-small.en.bin",
    "sha256":
      "c6138d6d58ecc8322097e0f987c32f1be8bb0a18532a3f88f734d1bbf9c41e5d",
    "approxSize": "488 MB",
    quantizeType: "full",
    isEnglishOnly: true,
  },
  {
    "name": "Small Full",
    "relativePath": "ggml-small.bin",
    "sha256":
      "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b",
    "approxSize": "488 MB",
    quantizeType: "full",
  },
  {
    "name": "Large V3 High Compression",
    "relativePath": "ggml-medium-q5_0.bin",
    "sha256":
      "19fea4b380c3a618ec4723c3eef2eb785ffba0d0538cf43f8f235e7b3b34220f",
    "approxSize": "539 MB",
    quantizeType: "q5",
  },
  {
    "name": "Large V3 Turbo (High Compression)",
    "relativePath": "ggml-large-v3-turbo-q5_0.bin",
    "sha256":
      "394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2",
    "approxSize": "574 MB",
    quantizeType: "q5",
  },
  {
    "name": "Large V3 Turbo (Medium Compression)",
    "relativePath": "ggml-large-v3-turbo-q8_0.bin",
    "sha256":
      "317eb69c11673c9de1e1f0d459b253999804ec71ac4c23c17ecf5fbe24e259a1",
    "approxSize": "874 MB",
    quantizeType: "q8",
  },
  {
    "name": "Medium Full (English-Only)",
    "relativePath": "ggml-medium.en.bin",
    "sha256":
      "cc37e93478338ec7700281a7ac30a10128929eb8f427dda2e865faa8f6da4356",
    "approxSize": "1.53 GB",
    quantizeType: "full",
    isEnglishOnly: true,
  },
  {
    "name": "Medium Full",
    "relativePath": "ggml-medium.bin",
    "sha256":
      "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208",
    "approxSize": "1.53 GB",
    quantizeType: "full",
  },
  {
    "name": "Large V3 Turbo",
    "relativePath": "ggml-large-v3-turbo.bin",
    "sha256":
      "1fc70f774d38eb169993ac391eea357ef47c88757ef72ee5943879b7e8e2bc69",
    "approxSize": "1.62 GB",
    quantizeType: "full",
  },
  {
    "name": "Large V2 Full",
    "relativePath": "ggml-large-v2.bin",
    "sha256":
      "9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487",
    "approxSize": "3.09 GB",
    quantizeType: "full",
    isSuperceded: true,
  },
  {
    "name": "Large V2 Full",
    "relativePath": "ggml-large-v2.bin",
    "sha256":
      "9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487",
    "approxSize": "3.09 GB",
    quantizeType: "full",
    isSuperceded: true,
  },
  {
    "name": "Large V3 Full",
    "relativePath": "ggml-large-v3.bin",
    "sha256":
      "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2",
    "approxSize": "3.1 GB",
    quantizeType: "full",
  },
  //   TODO(@): Get the rest of the models
];
