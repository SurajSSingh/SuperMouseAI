<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { blobChunksToBytes } from "$lib/myUtils";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { configStore } from "$lib/store";

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
    let currentTranscriptionIndex: number = $state(0);
    let transcriptions: string[] = $state([]);

    function nextTranscription() {
        if (currentTranscriptionIndex < transcriptions.length - 1)
            currentTranscriptionIndex++;
        transcribedOutput = transcriptions[currentTranscriptionIndex];
    }

    function previousTranscription() {
        if (currentTranscriptionIndex > 0) currentTranscriptionIndex--;
        transcribedOutput = transcriptions[currentTranscriptionIndex];
    }

    function addTranscription(transcript: string) {
        transcriptions.push(transcript);
        currentTranscriptionIndex = transcriptions.length - 1;
        transcribedOutput = transcriptions[currentTranscriptionIndex];
        configStore.setPrevTranscriptions(transcriptions);
    }

    function removeTranscription() {
        transcriptions.splice(currentTranscriptionIndex, 1);
        transcribedOutput = transcriptions[currentTranscriptionIndex];
        configStore.setPrevTranscriptions(transcriptions);
    }

    export async function processData(
        blobChunks?: Blob[],
        threads?: number,
        initialPrompt?: string,
        wordsToIgnore: string[] = [],
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
                ).trim();
                for (const word of wordsToIgnore) {
                    transcribedOutput = transcribedOutput.replaceAll(word, "");
                }
            }
            addTranscription(transcribedOutput);
            onFinishProcessing?.(transcribedOutput);
        } catch (error) {
            onError?.(`An error occured while transcribing: ${error}`);
        }
    }

    $effect(() => {
        const getTranscriptions = async () => {
            const previous = await configStore.getPrevTranscriptions();
            if (previous) {
                transcriptions = previous;
            }
        };
        getTranscriptions();
    });
</script>

<fieldset class="fieldset my-4 relative">
    <legend class="fieldset-legend">Transcription Output</legend>
    <div class="sm:absolute sm:-top-10 sm:right-0">
        <Button
            width="default"
            color="secondary"
            onclick={previousTranscription}
            class="text-xs"
            disabled={currentTranscriptionIndex === 0}>{"<"}Previous</Button
        >
        <span
            >{transcriptions.length
                ? currentTranscriptionIndex + 1
                : 0}/{transcriptions.length}</span
        >
        <Button
            width="default"
            color="secondary"
            onclick={nextTranscription}
            disabled={transcriptions.length === 0 ||
                currentTranscriptionIndex === transcriptions.length - 1}
            >Next{">"}</Button
        >
    </div>
    <Button
        variant="ghost"
        color="destructive"
        onclick={removeTranscription}
        disabled={transcriptions.length === 0}>Delete Current</Button
    >
    <Textarea
        color={transcriptions.length > 0 ? "success" : "warning"}
        size="md"
        class="rounded-md border-4 min-h-32 placeholder:text-center placeholder:text-xl placeholder:italic text-lg"
        placeholder="Record voice to transcribe..."
        bind:value={() => transcriptions[currentTranscriptionIndex],
        (v) => {
            transcriptions[currentTranscriptionIndex] = v;
        }}
        disabled={transcriptions.length === 0}
    />
</fieldset>
