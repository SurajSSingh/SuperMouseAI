import { BaseDirectory } from "@tauri-apps/api/path";
import type { WhisperModelInfo } from "./types.ts";

/**
 * The default model package with the app when installed
 */
export const DEFAULT_MODEL: WhisperModelInfo = {
  relativePath: "ggml-tiny-q5_1.bin",
  approxSize: 32152673,
  sha256: "818710568da3ca15689e31a743197b520007872ff9576237bda97bd1b469c3d7",
  modelSize: "tiny",
  quantizeType: "q5",
  isEnglishOnly: false,
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
    relativePath: "ggml-tiny.en-q5_1.bin",
    approxSize: 32166155,
    sha256: "c77c5766f1cef09b6b7d47f21b546cbddd4157886b3b5d6d4f709e91e66c7c2b",
    modelSize: "tiny",
    quantizeType: "q5",
    isEnglishOnly: true,
    isSuperceded: false,
  },
  {
    relativePath: "ggml-tiny-q8_0.bin",
    approxSize: 43537433,
    sha256: "c2085835d3f50733e2ff6e4b41ae8a2b8d8110461e18821b09a15c40c42d1cca",
    modelSize: "tiny",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 256_000_000, // 256MB
    recommendedRamForCPU: 512_000_000, // 512MB
  },
  {
    relativePath: "ggml-tiny.en-q8_0.bin",
    approxSize: 43550795,
    sha256: "5bc2b3860aa151a4c6e7bb095e1fcce7cf12c7b020ca08dcec0c6d018bb7dd94",
    modelSize: "tiny",
    quantizeType: "q8",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 256_000_000, // 256MB
    recommendedRamForCPU: 512_000_000, // 512MB
  },
  {
    relativePath: "ggml-base-q5_1.bin",
    approxSize: 59707625,
    sha256: "422f1ae452ade6f30a004d7e5c6a43195e4433bc370bf23fac9cc591f01a8898",
    modelSize: "base",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 512_000_000, // 512MB
    recommendedRamForCPU: 1_000_000_000, // 1GB
  },
  {
    relativePath: "ggml-base.en-q5_1.bin",
    approxSize: 59721011,
    sha256: "4baf70dd0d7c4247ba2b81fafd9c01005ac77c2f9ef064e00dcf195d0e2fdd2f",
    modelSize: "base",
    quantizeType: "q5",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 512_000_000, // 512MB
    recommendedRamForCPU: 1_000_000_000, // 1GB
  },
  {
    relativePath: "ggml-tiny.bin",
    approxSize: 77691713,
    sha256: "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21",
    modelSize: "tiny",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 750_000_000, // 0.75GB
    recommendedRamForCPU: 1_500_000_000, // 1.5GB
  },
  {
    relativePath: "ggml-tiny.en.bin",
    approxSize: 77704715,
    sha256: "921e4cf8686fdd993dcd081a5da5b6c365bfde1162e72b08d75ac75289920b1f",
    modelSize: "tiny",
    quantizeType: "full",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 750_000_000, // 0.75GB
    recommendedRamForCPU: 1_500_000_000, // 1.5GB
  },
  {
    relativePath: "ggml-base-q8_0.bin",
    approxSize: 81768585,
    sha256: "c577b9a86e7e048a0b7eada054f4dd79a56bbfa911fbdacf900ac5b567cbb7d9",
    modelSize: "base",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 750_000_000, // 0.75GB
    recommendedRamForCPU: 1_500_000_000, // 1.5GB
  },
  {
    relativePath: "ggml-base.en-q8_0.bin",
    approxSize: 81781811,
    sha256: "a4d4a0768075e13cfd7e19df3ae2dbc4a68d37d36a7dad45e8410c9a34f8c87e",
    modelSize: "base",
    quantizeType: "q8",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 750_000_000, // 0.75GB
    recommendedRamForCPU: 1_500_000_000, // 1.5GB
  },
  {
    relativePath: "ggml-base.bin",
    approxSize: 147951465,
    sha256: "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe",
    modelSize: "base",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 1073741824, // 1GB
    recommendedRamForCPU: 2147483648, // 2GB
  },
  {
    relativePath: "ggml-base.en.bin",
    approxSize: 147964211,
    sha256: "a03779c86df3323075f5e796cb2ce5029f00ec8869eee3fdfb897afe36c6d002",
    modelSize: "base",
    quantizeType: "full",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 1073741824, // 1GB
    recommendedRamForCPU: 2147483648, // 2GB
  },
  {
    relativePath: "ggml-small-q5_1.bin",
    approxSize: 190085487,
    sha256: "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb",
    modelSize: "small",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 1572864000, // 1.5GB
    recommendedRamForCPU: 2684354560, // 2.5GB
  },
  {
    relativePath: "ggml-small.en-q5_1.bin",
    approxSize: 190098681,
    sha256: "bfdff4894dcb76bbf647d56263ea2a96645423f1669176f4844a1bf8e478ad30",
    modelSize: "small",
    quantizeType: "q5",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 1572864000, // 1.5GB
    recommendedRamForCPU: 2684354560, // 2.5GB
  },
  {
    relativePath: "ggml-small-q8_0.bin",
    approxSize: 264464607,
    sha256: "49c8fb02b65e6049d5fa6c04f81f53b867b5ec9540406812c643f177317f779f",
    modelSize: "small",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 1572864000, // 1.5GB
    recommendedRamForCPU: 3145728000, // 3GB
  },
  {
    relativePath: "ggml-small.en-q8_0.bin",
    approxSize: 264477561,
    sha256: "67a179f608ea6114bd3fdb9060e762b588a3fb3bd00c4387971be4d177958067",
    modelSize: "small",
    quantizeType: "q8",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 1572864000, // 1.5GB
    recommendedRamForCPU: 3145728000, // 3GB
  },
  {
    relativePath: "ggml-small.bin",
    approxSize: 487601967,
    sha256: "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b",
    modelSize: "small",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 2147483648, // 2GB
    recommendedRamForCPU: 4294967296, // 4GB
  },
  {
    relativePath: "ggml-small.en.bin",
    approxSize: 487614201,
    sha256: "c6138d6d58ecc8322097e0f987c32f1be8bb0a18532a3f88f734d1bbf9c41e5d",
    modelSize: "small",
    quantizeType: "full",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 2147483648, // 2GB
    recommendedRamForCPU: 4294967296, // 4GB
  },
  {
    relativePath: "ggml-medium-q5_0.bin",
    approxSize: 539212467,
    sha256: "19fea4b380c3a618ec4723c3eef2eb785ffba0d0538cf43f8f235e7b3b34220f",
    modelSize: "medium",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 2147483648, // 2GB
    recommendedRamForCPU: 4294967296, // 4GB
  },
  {
    relativePath: "ggml-medium.en-q5_0.bin",
    approxSize: 539225533,
    sha256: "76733e26ad8fe1c7a5bf7531a9d41917b2adc0f20f2e4f5531688a8c6cd88eb0",
    modelSize: "medium",
    quantizeType: "q5",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 2147483648, // 2GB
    recommendedRamForCPU: 4294967296, // 4GB
  },
  {
    relativePath: "ggml-large-v3-turbo-q5_0.bin",
    approxSize: 574041195,
    sha256: "394221709cd5ad1f40c46e6031ca61bce88931e6e088c188294c6d5a55ffa7e2",
    modelSize: "large",
    version: "v3 turbo",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 2147483648, // 2GB
    recommendedRamForCPU: 4294967296, // 4GB
  },
  {
    relativePath: "ggml-medium-q8_0.bin",
    approxSize: 823369779,
    sha256: "42a1ffcbe4167d224232443396968db4d02d4e8e87e213d3ee2e03095dea6502",
    modelSize: "medium",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 3221225472, // 3GB
    recommendedRamForCPU: 6442450944, // 6GB
  },
  {
    relativePath: "ggml-medium.en-q8_0.bin",
    approxSize: 823382461,
    sha256: "43fa2cd084de5a04399a896a9a7a786064e221365c01700cea4666005218f11c",
    modelSize: "medium",
    quantizeType: "q8",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 3221225472, // 3GB
    recommendedRamForCPU: 6442450944, // 6GB
  },
  {
    relativePath: "ggml-large-v3-turbo-q8_0.bin",
    approxSize: 874188075,
    sha256: "317eb69c11673c9de1e1f0d459b253999804ec71ac4c23c17ecf5fbe24e259a1",
    modelSize: "large",
    version: "v3 turbo",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 3221225472, // 3GB
    recommendedRamForCPU: 6442450944, // 6GB
  },
  {
    relativePath: "ggml-large-v2-q5_0.bin",
    approxSize: 1080732091,
    sha256: "3a214837221e4530dbc1fe8d734f302af393eb30bd0ed046042ebf4baf70f6f2",
    modelSize: "large",
    version: "v2",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: true,
    recommendedVramForGPU: 4294967296, // 4GB
    recommendedRamForCPU: 8589934592, // 8GB
  },
  {
    relativePath: "ggml-large-v3-q5_0.bin",
    approxSize: 1081140203,
    sha256: "d75795ecff3f83b5faa89d1900604ad8c780abd5739fae406de19f23ecd98ad1",
    modelSize: "large",
    version: "v3",
    quantizeType: "q5",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 4294967296, // 4GB
    recommendedRamForCPU: 8589934592, // 8GB
  },
  {
    relativePath: "ggml-medium.bin",
    approxSize: 1533763059,
    sha256: "6c14d5adee5f86394037b4e4e8b59f1673b6cee10e3cf0b11bbdbee79c156208",
    modelSize: "medium",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 6442450944, // 6GB
    recommendedRamForCPU: 12884901888, // 12GB
  },
  {
    relativePath: "ggml-medium.en.bin",
    approxSize: 1533774781,
    sha256: "cc37e93478338ec7700281a7ac30a10128929eb8f427dda2e865faa8f6da4356",
    modelSize: "medium",
    quantizeType: "full",
    isEnglishOnly: true,
    isSuperceded: false,
    recommendedVramForGPU: 6442450944, // 6GB
    recommendedRamForCPU: 12884901888, // 12GB
  },
  {
    relativePath: "ggml-large-v3-turbo.bin",
    approxSize: 1624555275,
    sha256: "1fc70f774d38eb169993ac391eea357ef47c88757ef72ee5943879b7e8e2bc69",
    modelSize: "large",
    version: "v3 turbo",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 6442450944, // 6GB
    recommendedRamForCPU: 12884901888, // 12GB
  },
  {
    relativePath: "ggml-large-v2-q8_0.bin",
    approxSize: 1656129691,
    sha256: "fef54e6d898246a65c8285bfa83bd1807e27fadf54d5d4e81754c47634737e8c",
    modelSize: "large",
    version: "v2",
    quantizeType: "q8",
    isEnglishOnly: false,
    isSuperceded: true,
    recommendedVramForGPU: 6442450944, // 6GB
    recommendedRamForCPU: 12884901888, // 12GB
  },
  {
    relativePath: "ggml-large-v1.bin",
    approxSize: 3094623691,
    sha256: "7d99f41a10525d0206bddadd86760181fa920438b6b33237e3118ff6c83bb53d",
    modelSize: "large",
    version: "v1",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: true,
    recommendedVramForGPU: 12884901888, // 12GB
    recommendedRamForCPU: 25769803776, // 24GB
  },
  {
    relativePath: "ggml-large-v2.bin",
    approxSize: 3094623691,
    sha256: "9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487",
    modelSize: "large",
    version: "v2",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: true,
    recommendedVramForGPU: 12884901888, // 12GB
    recommendedRamForCPU: 25769803776, // 24GB
  },
  {
    relativePath: "ggml-large-v3.bin",
    approxSize: 3095033483,
    sha256: "64d182b440b98d5203c4f9bd541544d84c605196c4f7b845dfa11fb23594d1e2",
    modelSize: "large",
    version: "v3",
    quantizeType: "full",
    isEnglishOnly: false,
    isSuperceded: false,
    recommendedVramForGPU: 12884901888, // 12GB
    recommendedRamForCPU: 25769803776, // 24GB
  },
];
