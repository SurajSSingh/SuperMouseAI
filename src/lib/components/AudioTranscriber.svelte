<script lang="ts">
    import { blobChunksToBytes } from "$lib/myUtils";
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { configStore } from "$lib/store.svelte";
    import {
        notifier,
        type NotificationSystem,
    } from "$lib/notificationSystem.svelte";
    import { commands } from "$lib/bindings";

    interface TranscriberProps {
        onFinishProcessing?: (text: string) => void;
        onError?: (err: string) => void;
    }

    let { onFinishProcessing, onError }: TranscriberProps = $props();

    let workingChunks: Blob[] = $state([]);

    export async function processData(blobChunks?: Blob[]) {
        try {
            workingChunks = blobChunks ?? workingChunks;
            if (workingChunks.length > 0) {
                let result = await commands.transcribeWithPostProcess(
                    // @ts-ignore Uint8Array should be number[]-like
                    await blobChunksToBytes(workingChunks),
                    {
                        threads:
                            configStore.threads.value > 0
                                ? configStore.threads.value
                                : null,
                        initial_prompt: configStore.initialPrompt.value,
                    },
                    {
                        removed_words: configStore.ignoredWordsList,
                        replace_inter_sentence_newlines:
                            configStore.interNLRemove.value,
                    },
                );
                if (result.status === "error") {
                    onError?.(
                        `An error occured while transcribing: ${result.error}`,
                    );
                    return;
                }
                configStore.addTranscription(result.data);
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
            disabled={configStore.currentIndex.value === 0}
            >{"<"}Previous</Button
        >
        <span
            >{configStore.transcriptLength
                ? configStore.currentIndex.value + 1
                : 0}/{configStore.transcriptLength}</span
        >
        <Button
            width="default"
            color="secondary"
            onclick={() => configStore.nextIndex()}
            disabled={configStore.isTranscriptsEmpty ||
                configStore.currentIndex.value ===
                    configStore.transcriptions.value.length - 1}
            >Next{">"}</Button
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
        color={configStore.transcriptions.value.length > 0
            ? "success"
            : "warning"}
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
