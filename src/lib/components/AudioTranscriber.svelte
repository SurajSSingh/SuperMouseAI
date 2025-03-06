<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { blobChunksToBytes } from "$lib/myUtils";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { configStore } from "$lib/store.svelte";

    interface TranscriberProps {
        onFinishProcessing?: (text: string) => void;
        onError?: (err: string) => void;
    }

    let { onFinishProcessing, onError }: TranscriberProps = $props();

    let workingChunks: Blob[] = $state([]);

    // function nextTranscription() {
    //     if (currentTranscriptionIndex < configStore.transcriptions.length - 1)
    //         currentTranscriptionIndex++;
    //     transcribedOutput =
    //         configStore.transcriptions[currentTranscriptionIndex];
    // }

    // function previousTranscription() {
    //     if (currentTranscriptionIndex > 0) currentTranscriptionIndex--;
    //     transcribedOutput =
    //         configStore.transcriptions[currentTranscriptionIndex];
    // }

    // function addTranscription(transcript: string) {
    //     // configStore.transcriptions.push(transcript);
    //     configStore.addTranscription(transcript);
    //     currentTranscriptionIndex = configStore.transcriptions.length - 1;
    //     transcribedOutput =
    //         configStore.transcriptions[currentTranscriptionIndex];
    // }

    // function removeTranscription() {
    //     // configStore.transcriptions.splice(currentTranscriptionIndex, 1);
    //     configStore.removeTranscription(currentTranscriptionIndex);
    //     transcribedOutput =
    //         configStore.transcriptions[currentTranscriptionIndex];
    // }

    export async function processData(
        blobChunks?: Blob[],
        threads?: number,
        initialPrompt?: string,
        wordsToIgnore: string[] = [],
    ) {
        try {
            workingChunks = blobChunks ?? workingChunks;
            if (workingChunks.length > 0) {
                let transcribedResult = (
                    (await invoke("transcribe", {
                        audioData: await blobChunksToBytes(workingChunks),
                        threads,
                        initialPrompt,
                    })) as string
                ).trim();
                for (const word of wordsToIgnore) {
                    transcribedResult = transcribedResult.replaceAll(word, "");
                }
                configStore.addTranscription(transcribedResult);
            }
            onFinishProcessing?.(configStore.currentTranscript);
        } catch (error) {
            onError?.(`An error occured while transcribing: ${error}`);
        }
    }
</script>

<fieldset class="fieldset my-4 relative">
    <legend class="fieldset-legend">Transcription Output</legend>
    <div class="sm:absolute sm:-top-10 sm:right-0">
        <Button
            width="default"
            color="secondary"
            onclick={() => configStore.prevIndex()}
            class="text-xs"
            disabled={configStore.currentTranscriptionIndex === 0}
            >{"<"}Previous</Button
        >
        <span
            >{configStore.transcriptLength
                ? configStore.currentTranscriptionIndex + 1
                : 0}/{configStore.transcriptLength}</span
        >
        <Button
            width="default"
            color="secondary"
            onclick={() => configStore.nextIndex()}
            disabled={configStore.isTranscriptsEmpty ||
                configStore.currentTranscriptionIndex ===
                    configStore.transcriptions.length - 1}>Next{">"}</Button
        >
    </div>
    <Button
        variant="ghost"
        color="destructive"
        onclick={() => configStore.removeCurrentTranscription()}
        disabled={configStore.isTranscriptsEmpty}>Delete Current</Button
    >
    <Textarea
        color={configStore.transcriptions.length > 0 ? "success" : "warning"}
        size="md"
        class="rounded-md border-4 min-h-32 placeholder:text-center placeholder:text-xl placeholder:italic text-lg"
        placeholder="Record voice to transcribe..."
        bind:value={() => configStore.currentTranscript,
        (v) => {
            configStore.editTranscription(v);
        }}
        disabled={configStore.isTranscriptsEmpty}
    />
</fieldset>
