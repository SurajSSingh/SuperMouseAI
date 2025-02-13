<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { blobChunksToBytes } from "./utils";

    interface TranscriberProps {
        transcribedOutput: string;
        onFinishProcessing?: (text: string) => void;
        onError?: (err: string) => void;
    }

    let {
        transcribedOutput = $bindable(""),
        onFinishProcessing,
        onError,
    }: TranscriberProps = $props();

    export async function processData(blobChunks: Blob[]) {
        try {
            transcribedOutput = (
                (await invoke("transcribe", {
                    audioData: await blobChunksToBytes(blobChunks),
                })) as string
            )
                .trim()
                .replaceAll("[BLANK_AUDIO]", "");
            onFinishProcessing?.(transcribedOutput);
        } catch (error) {
            onError?.(`An error occured while transcribing: ${error}`);
        }
    }
</script>

<textarea
    class="mx-32 my-4 text-center rounded-md border-4 min-h-32"
    placeholder="This is where the output goes"
    disabled={transcribedOutput === ""}>{transcribedOutput}</textarea
>
