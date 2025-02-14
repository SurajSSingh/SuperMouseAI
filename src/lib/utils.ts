/**
   * Convert a blob to array of bytes
   */
export async function blobToBytes(blob: Blob): Promise<Uint8Array> {
    if (blob.bytes) return blob.bytes();
    // Fallback to making bytes from array buffer
    return new Uint8Array(await blob.arrayBuffer());
  }

export async function blobChunksToBytes(chunks:Blob[], type = "audio/wav"): Promise<Uint8Array> {
    const blob = chunks.length === 1 ? chunks[0]! : new Blob(chunks, {type:type});
    return blobToBytes(blob);
    
}