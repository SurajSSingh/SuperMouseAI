<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { blobChunksToBytes } from "$lib/myUtils";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { configStore } from "$lib/store.svelte";
    import type { NotificationSystem } from "$lib/notificationSystem.svelte";

    interface TranscriberProps {
        onFinishProcessing?: (text: string) => void;
        onError?: (err: string) => void;
        notifier?: NotificationSystem;
    }

    let { onFinishProcessing, onError, notifier }: TranscriberProps = $props();

    let workingChunks: Blob[] = $state([]);

    export async function processData(blobChunks?: Blob[]) {
        try {
            workingChunks = blobChunks ?? workingChunks;
            if (workingChunks.length > 0) {
                let transcribedResult = (
                    (await invoke("transcribe", {
                        audioData: await blobChunksToBytes(workingChunks),
                        threads: configStore.threads,
                        initialPrompt: configStore.initialPrompt,
                    })) as string
                )
                    .trim()
                    // Replace all "empty" newlines after words
                    .replaceAll(/(?<=\w)[ \t]*\n/g, " ");
                for (const word of configStore.ignoredWordsList) {
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
        onclick={() => {
            notifier?.confirmAction(
                `You are deleting: ${configStore.currentTranscript.length > 100 ? configStore.currentTranscript.substring(0, 100).trimEnd() + "..." : configStore.currentTranscript}`,
                () => {
                    configStore.removeCurrentTranscription();
                },
                () => {
                    notifier?.showInfo("Cancelled delete.", "", "");
                },
                "Are you sure you want to delete?",
            );
        }}
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
