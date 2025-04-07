import { debug, trace, warn } from "@tauri-apps/plugin-log";
import {
  BASE_LOCAL_APP_DIR,
  MODELS_DIR,
  WHISPER_GGML_MODELS,
} from "./constants.ts";
import { exists } from "@tauri-apps/plugin-fs";
import type { WhisperModelInfo } from "./types.ts";

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
