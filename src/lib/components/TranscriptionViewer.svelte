<script lang="ts">
    import Textarea from "$lib/components/ui/textarea/textarea.svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import { configStore } from "$lib/store.svelte";
    import { notifier } from "$lib/notificationSystem.svelte";
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
                if (v.length === 0) {
                    info(
                        "User attempting to fully delete text, making sure that is what they want.",
                    );
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
                } else {
                    configStore.editTranscription(v);
                }
            }
        }
        disabled={configStore.isTranscriptsEmpty}
    />
    {#if configStore.currentTranscriptObject !== null}
        <details>
            <summary class="text-md hover:text-lg">Transcription Info</summary>
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
</fieldset>
