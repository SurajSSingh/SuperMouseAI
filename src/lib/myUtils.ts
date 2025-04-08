import { debug, trace, warn } from "@tauri-apps/plugin-log";
import {
  BASE_LOCAL_APP_DIR,
  MODELS_DIR,
  WHISPER_GGML_MODELS,
} from "./constants.ts";
import { exists } from "@tauri-apps/plugin-fs";
import type { WhisperModelInfo } from "./types.ts";
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
        ),
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
export function nameOfModel(model?: WhisperModelInfo): string {
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
 * Determines the largest model based on what the user can run.
 * @param systemInfo Information about the user's system
 * @returns The largest model that works on the system; `null` means use default
 */
export function findLargestUseableModel(
  systemInfo: SystemInfo,
): WhisperModelInfo | null {
  // TODO(@): Find best model
  return WHISPER_GGML_MODELS.at(0) || null;
}
