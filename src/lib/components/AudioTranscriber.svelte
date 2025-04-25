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
    import { debug, error } from "@tauri-apps/plugin-log";

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
                debug(
                    `Call command to transcribe audio and then process text.`,
                );
                let result = await commands.transcribeWithPostProcess(
                    // @ts-ignore Uint8Array should be number[]-like
                    await blobChunksToBytes(workingChunks),
                    {
                        threads:
                            configStore.threads.value > 0
                                ? configStore.threads.value
                                : null,
                        initial_prompt: configStore.initialPrompt.value,
                        patience: configStore.patience.value,
                        include_callback: false,
                    },
                    {
                        removed_words: configStore.ignoredWordsList,
                        replace_inter_sentence_newlines:
                            configStore.interNLRemove.value,
                    },
                    {
                        denoise_audio: true,
                        normalize_result: true,
                        low_pass_value: 3000,
                        high_pass_value: 200,
                    },
                );
                debug(`Finish `);
                if (result.status === "error") {
                    onError?.(
                        `An error occured while transcribing/processing data: ${result.error}`,
                    );
                    return;
                }
                configStore.addTranscription(result.data[0], result.data[1]);
            }
            onFinishProcessing?.(configStore.currentTranscript);
        } catch (err) {
            onError?.(
                `An error occured while transcribing/processing data: ${err}`,
            );
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
                    notifier?.showToast("Cancelled delete.", "info");
                },
                "Are you sure you want to delete?",
            );
        }}
        disabled={configStore.isTranscriptsEmpty}>Delete Current</Button
    >
    {#if configStore.currentTranscriptObject !== null}
        <details>
            <summary>Transcription Info</summary>
            <p>
                Model used: {configStore.currentTranscriptObject.model ||
                    "Default"}
            </p>
            <p>
                Model Provider: {configStore.currentTranscriptObject.provider ||
                    "whisper-cpp"}
            </p>
            <p>
                GPU used? {configStore.currentTranscriptObject.onGPU || true}
            </p>
            <p>
                Processing Time: {configStore.currentTranscriptObject.processingTime?.toFixed(
                    3,
                ) || "???"} seconds
            </p>
            {#if configStore.currentTranscriptObject.strategy?.type === "beam"}
                <p>Strategy: Beam</p>
                <p>
                    Decoders used: {configStore.currentTranscriptObject.strategy
                        .beamSize}
                </p>
                <p>
                    Patience: {configStore.currentTranscriptObject.strategy
                        .patience}
                </p>
            {:else if configStore.currentTranscriptObject.strategy?.type === "greedy"}
                <p>Strategy: Greedy</p>
                <p>
                    Decoders used: {configStore.currentTranscriptObject.strategy
                        .bestOf}
                </p>
                <p>
                    Temperature: {configStore.currentTranscriptObject.strategy
                        .temperature}
                </p>
            {/if}
        </details>
    {/if}
    <Textarea
        color={configStore.transcriptions.value.length > 0
            ? "success"
            : "warning"}
        size="md"
        class="rounded-md border-4 min-h-32 placeholder:text-center placeholder:text-xl placeholder:italic text-lg"
        placeholder="Record voice to transcribe..."
        bind:value={
            () => configStore.currentTranscript,
            (v) => {
                configStore.editTranscription(v);
            }
        }
        disabled={configStore.isTranscriptsEmpty}
    />
</fieldset>
