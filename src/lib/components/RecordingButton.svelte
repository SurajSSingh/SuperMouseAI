<script lang="ts">
    import { commands } from "$lib/bindings";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { Button } from "./ui/button";
    // import Loading from "./ui/Loading.svelte";

    interface RecorderProps {
        id?: string;
        // recordingState: RecordingStates;
        onRecordingStart?: () => void;
        onRecordingEnd?: (chunks: Blob[]) => void;
        onError?: (err: string) => void;
        disabled?: boolean;
    }

    let {
        id = "audio-holder",
        // recordingState = "stopped",
        onRecordingStart,
        onRecordingEnd,
        onError,
        disabled = false,
    }: RecorderProps = $props();

    let isRecording = $state(false);
    let isProcessing = $state(false);
    async function toggleRecording() {
        if (isProcessing) return;
        if (isRecording) {
            isRecording = false;
            isProcessing = true;
            const res = await commands.stopTranscribeAndProcessData(
                null,
                "Skip",
                {
                    normalize_result: true,
                    denoise_audio: null,
                    high_pass_value: null,
                    low_pass_value: null,
                },
            );
            if (res.status === "ok") {
                isProcessing = false;
                notifier.showToast(res.data, "success");
            } else {
                notifier.showToast(res.error, "error");
            }
        } else {
            const res = await commands.startMicrophoneRecording();
            if (res.status === "ok") {
                isRecording = true;
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
        disabled={disabled || isProcessing}>Test Recording</Button
    >
</section>
