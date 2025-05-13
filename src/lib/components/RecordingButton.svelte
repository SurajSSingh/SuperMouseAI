<script lang="ts">
    import { commands } from "$lib/bindings";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { debug } from "@tauri-apps/plugin-log";
    import { Button } from "./ui/button";
    import Loading from "./ui/Loading.svelte";

    interface RecorderProps {
        id?: string;
        // recordingState: RecordingStates;
        onRecordingStart?: () => void;
        onRecordingEnd?: () => Promise<void>;
        onError?: (err: string) => void;
        disabled?: boolean;
    }

    let {
        id = "audio-recorder",
        // recordingState = "stopped",
        onRecordingStart,
        onRecordingEnd,
        onError,
        disabled = false,
    }: RecorderProps = $props();

    let isRecording = $state(false);
    let isProcessing = $state(false);

    const buttonText = $derived(
        isRecording
            ? "Stop Recording"
            : isProcessing
              ? "Processing..."
              : "Start Recording",
    );
    const recordingText = $derived(
        isRecording
            ? "(🎤🔴): Microphone RECORDING"
            : isProcessing
              ? "(🗣️📝): Processing Audio"
              : "(🎤⏹️): Microphone Inactive",
    );
    export async function toggleRecording() {
        if (isProcessing) return;
        if (isRecording) {
            debug("Switch to processing mode.");
            isRecording = false;
            isProcessing = true;
            const stopped = await commands.stopMicrophoneRecording(1000);
            if (stopped.status === "error") {
                onError?.(`Stopping microphone failed: ${stopped.error}`);
                return;
            }
            await onRecordingEnd?.();
            // const res = await commands.transcribeCurrentData(null, {
            //     normalize_result: true,
            //     denoise_audio: null,
            //     high_pass_value: null,
            //     low_pass_value: null,
            // });
            // if (res.status === "ok") {
            //     notifier.showToast(res.data, "success");
            // } else {
            //     onError?.(`Processing failed: ${res.error}`);
            // }
            isProcessing = false;
        } else {
            const res = await commands.startMicrophoneRecording();
            if (res.status === "ok") {
                debug(
                    "Use result of audio recorder creation to signify recording",
                );
                isRecording = res.data;
                if (isRecording) onRecordingStart?.();
            } else {
                notifier.showToast(res.error, "error");
            }
        }
    }

    $inspect(isRecording, isProcessing);
</script>

<section {id} class="my-2 text-center">
    <Button
        class="p-2 w-3/4 my-2 font-semibold rounded-sm"
        color={isRecording
            ? "destructive"
            : isProcessing
              ? "warning"
              : "success"}
        size="sm"
        width="block"
        onclick={toggleRecording}
        disabled={disabled || isProcessing}>{buttonText}</Button
    >
    <p class="mb-2">
        {#if isProcessing}
            <Loading />
        {/if}
        {recordingText}
    </p>
</section>
