import { debug, error, trace, warn } from "@tauri-apps/plugin-log";
import {
  BASE_LOCAL_APP_DIR,
  MODELS_DIR,
  WHISPER_GGML_MODELS,
} from "./constants.ts";
import { exists } from "@tauri-apps/plugin-fs";
import type { LargestModelFinderOption, WhisperModelInfo } from "./types.ts";
import type { SystemInfo } from "./bindings.ts";

/**
 * Convert a blob to array of bytes
 */
export async function blobToBytes(blob: Blob): Promise<Uint8Array> {
  debug(`Converting blob of size ${blob.size} to bytes`);
  if (blob.bytes) return blob.bytes();
  warn(`No byte() function on blob, falling back to manual conversion`);
  // Fallback to making bytes from array buffer
  return new Uint8Array(await blob.arrayBuffer());
}

/**
 * Convert a blob to array of bytes with a given type
 */
export function blobChunksToBytes(
  chunks: Blob[],
  type = "audio/wav",
): Promise<Uint8Array> {
  debug(`Running blob chunks to bytes`);
  const blob = chunks.length === 1
    ? chunks[0]
    : new Blob(chunks, { type: type });
  trace(`Made a blob to convert`);
  return blobToBytes(blob);
}

/**
 * Find which models are currently downloaded
 * @returns a promise to an array of models and whether it is downloaded
 */
export function checkDownloadedModels(): Promise<
  { model: WhisperModelInfo; downloaded: boolean }[]
> {
  return Promise.all(
    WHISPER_GGML_MODELS.map(async (model) => {
      return {
        model,
        downloaded: await exists(
          `${MODELS_DIR}/${model.relativePath}`,
          BASE_LOCAL_APP_DIR,
        ).catch((err) => {
          error(`Could not check for model existence: ${err}`);
          return false;
        }),
      };
    }),
  );
}

const units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

/**
 * Convert bytes per second into more understandable format
 * @param bytesPerSecond
 * @param precision: how many decimal places to use
 * @returns converted into prefixed-bytes per second
 */
export function convertBPSToHuman(
  bytesPerSecond: number,
  precision = 2,
): string {
  // Convert as bytes, then add per second to the end
  return `${convertBytesToHuman(bytesPerSecond, precision)}ps`;
}

/**
 * Convert bytes into more understandable format
 * @param bytes
 * @param precision: how many decimal places to use
 * @returns  converted into prefixed-bytes
 */
export function convertBytesToHuman(bytes: number, precision = 2): string {
  if (!Number.isFinite(bytes) || Number.isNaN(bytes) || bytes <= 0) {
    return "0 B";
  }
  const powerBy10 = Math.log10(bytes);
  const powerBy1000 = Math.floor(powerBy10 / 3);
  const bytesUnit = units[powerBy1000];
  const formattedBytes = (bytes / Math.pow(1000, powerBy1000)).toFixed(
    precision,
  );
  return `${formattedBytes} ${bytesUnit}`;
}

/**
 * Get a the name of the model formatted for displaying to user
 * @param model Model to get name of
 * @returns The model's formatted name or "Unknown Model"
 */
export function nameOfModel(model?: WhisperModelInfo | null): string {
  if (!model) return "Unknown Model";
  const modelSize = model.modelSize.replace(
    /\b\w/g,
    (char) => char.toUpperCase(),
  );
  const version =
    model.version?.replace(/\b\w/g, (char) => " " + char.toUpperCase()) || "";
  const compression = model.quantizeType === "q8"
    ? " Low Compression"
    : model.quantizeType === "q5"
    ? " High Compression"
    : "";
  const langType = model.isEnglishOnly ? " (English Only)" : "";
  return `${modelSize}${version}${compression}${langType}`;
}

/**
 * Get the size of a model based on relative size (rather than raw bytes total)
 * @param model Model to get size of
 * @returns formatted string of human readable size
 */
export function sizeOfModel(model: WhisperModelInfo, precision = 2): string {
  return convertBytesToHuman(model.approxSize, precision);
}

/**
 * Determines the best Whisper model for a given system configuration.
 *
 * @param systemInfo User's system specifications
 * @param preferEnglishOnly Whether to prioritize English-only models (default: true)
 * @param usesCPU Whether model will run on CPU
 * @param compressionAllowed The level of quantized models that are allow
 * @param maxRAMRatio The minimum amount of CPU RAM a model is allowed to take
 * @param maxRAMRatio The maximum amount of CPU RAM a model is allowed to take
 * @param maxVRAMRatio The mimum amount of GPU VRAM a model is allowed to take
 * @param maxVRAMRatio The maximum amount of GPU VRAM a model is allowed to take
 * @returns The best matching model or null if none suitable
 */
export function findLargestUseableModel(
  systemInfo: SystemInfo,
  options: LargestModelFinderOption = {
    preferEnglishOnly: true,
    usesCPU: false,
    compressionAllowed: "all",
    minRAMRatio: 0.01,
    maxRAMRatio: 0.05,
    minVRAMRatio: 0.0,
    maxVRAMRatio: 0.75,
    sizePriority: {
      largeTurbo: 5,
      small: 4,
      large: 3,
      base: 2,
      medium: 1,
      tiny: 0,
    },
  },
): WhisperModelInfo | null {
  // Helper functions
  // Compression check
  const compressionCheck = (quantizeType: "full" | "q8" | "q5") => {
    return options.compressionAllowed === "all" || quantizeType === "full" ||
      (options.compressionAllowed === "low" && quantizeType !== "q5");
  };
  // Define the minimum requirements
  const meetsMinimumRequirements = (model: WhisperModelInfo): boolean => {
    const modelGBSize = Math.ceil(model.approxSize / 1_000_000_000);
    return (
      !model.isSuperceded &&
      (model.isEnglishOnly || !options.preferEnglishOnly) &&
      compressionCheck(model.quantizeType) &&
      modelGBSize >= systemInfo.total_vram_gb * options.minVRAMRatio &&
      modelGBSize <= systemInfo.total_vram_gb * options.maxVRAMRatio &&
      (!options.usesCPU ||
        (modelGBSize >= systemInfo.total_memory_gb * options.minRAMRatio &&
          modelGBSize <= systemInfo.total_memory_gb * options.maxRAMRatio)) &&
      // Non-tiny model require multi-processing
      (model.modelSize === "tiny" || systemInfo.cpu_core_count >= 4)
      // TODO(@): Consider Disk Space
    );
  };
  // Get priority of a model
  const getModelPriority = (model: WhisperModelInfo) =>
    model.version === "v3 turbo"
      ? options.sizePriority.largeTurbo
      : options.sizePriority[model.modelSize];

  // Get quantized priority of a model
  const getQuantPriority = (model: WhisperModelInfo) =>
    model.quantizeType === "full" ? 2 : model.quantizeType === "q8" ? 1 : 0;

  debug("Finding suitable models");
  const suitableModels = WHISPER_GGML_MODELS.filter(meetsMinimumRequirements);
  if (suitableModels.length === 0) {
    debug("No model is suitable");
    return null;
  }
  // Get the smallest model that works
  console.log(suitableModels.sort((a, b) => a.approxSize - b.approxSize));
  console.log(
    suitableModels.sort((a, b) => getQuantPriority(b) - getQuantPriority(a)),
  );
  console.log(
    suitableModels.sort((a, b) => getModelPriority(b) - getModelPriority(a)),
  );
  return suitableModels.at(0) || null;
}
