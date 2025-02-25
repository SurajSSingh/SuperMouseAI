<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { blobChunksToBytes } from "./utils";
    import Textarea from "./components/ui/textarea/textarea.svelte";

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

    let workingChunks: Blob[] = $state([]);

    export async function processData(
        blobChunks?: Blob[],
        threads?: number,
        initialPrompt?: string,
    ) {
        try {
            workingChunks = blobChunks ?? workingChunks;
            if (workingChunks.length > 0) {
                transcribedOutput = (
                    (await invoke("transcribe", {
                        audioData: await blobChunksToBytes(workingChunks),
                        threads,
                        initialPrompt,
                    })) as string
                )
                    .trim()
                    .replaceAll("[BLANK_AUDIO]", "");
            }
            onFinishProcessing?.(transcribedOutput);
        } catch (error) {
            onError?.(`An error occured while transcribing: ${error}`);
        }
    }
</script>

<fieldset class="fieldset my-4">
    <legend class="fieldset-legend">Transcription Output</legend>
    <Textarea
        color={transcribedOutput ? "success" : "warning"}
        size="xl"
        class="text-center rounded-md border-4 min-h-32 placeholder:text-xl placeholder:italic text-lg"
        placeholder="Record voice to transcribe..."
        bind:value={transcribedOutput}
        disabled={workingChunks.length === 0}
    />
</fieldset>
