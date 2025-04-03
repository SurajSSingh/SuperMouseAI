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
