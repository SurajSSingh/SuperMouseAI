<script lang="ts">
    import type { RecordingStates } from "./types";
    import {
        MediaRecorder as ExtendedMediaRecorder,
        register,
        deregister,
        type IMediaRecorder,
    } from "extendable-media-recorder";
    import { connect } from "extendable-media-recorder-wav-encoder";

    interface RecorderProps {
        id?: string;
        recordingState: RecordingStates;
        onRecordingStart?: () => void;
        onRecordingEnd?: (chunks: Blob[]) => void;
        onError?: (err: string) => void;
    }

    let {
        id = "audio-holder",
        recordingState = $bindable("stopped"),
        onRecordingStart,
        onRecordingEnd,
        onError,
    }: RecorderProps = $props();

    // Regular Variables
    let wavRecorderConnection: MessagePort | undefined;
    let audioChunks: Blob[] = [];

    // State
    let audioStream: MediaStream | null = $state(null);
    let audioRecorder: IMediaRecorder | null = $state(null);

    // Derived values
    const isRecording = $derived(recordingState === "recording");
    const isProcessing = $derived(recordingState === "processing");
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
              ? "(🗣️📝): Processing File"
              : "(🎤⏹️): Microphone Inactive",
    );

    // Effects
    $effect(() => {
        // On-Mount
        setupRecorder();

        return async () => {
            // Clean-up code
            audioRecorder = null;
            audioStream = null;
            if (wavRecorderConnection) deregister(wavRecorderConnection);
        };
    });

    export async function setupRecorder() {
        try {
            wavRecorderConnection = await connect();
            await register(wavRecorderConnection);
            audioStream = await navigator.mediaDevices.getUserMedia({
                audio: {
                    autoGainControl: false,
                    noiseSuppression: true,
                    echoCancellation: true,
                },
                video: false,
            });
        } catch (error) {
            onError?.(`Error on setting up recorder: ${error}`);
        }
    }

    function startRecording() {
        if (isProcessing) {
            return;
        }
        if (!audioStream) {
            onError?.("Audio Stream not setup, stopping recording!");
            return;
        }
        try {
            // Create a new audio to start recording
            // REASON: Maybe better security as only
            // record when user want to start recording
            audioRecorder = new ExtendedMediaRecorder(audioStream, {
                mimeType: "audio/wav",
            });
            audioRecorder.ondataavailable = (blobEvent) => {
                audioChunks.push(blobEvent.data);
            };
            audioRecorder.onstart = (_e) => {
                // Clear old recording
                audioChunks = [];
            };
            audioRecorder.onstop = (_e) => {
                // Delete audio recorder to stop listening
                audioRecorder = null;

                onRecordingEnd?.(audioChunks);
            };

            // Call callback function to start recording
            // REASON: If error occurs in callback function,
            // can prevent recording from starting,
            // allowing (potentially) better security
            onRecordingStart?.();

            audioRecorder.start();
        } catch (error: any) {
            onError?.(
                `Stopping recording, ran into an error: ${error.toString()}`,
            );
        }
    }

    function stopRecording() {
        if (!isRecording) {
            return;
        }
        if (audioRecorder === null) {
            onError?.("No Audio recorder active!");
            return;
        }
        audioRecorder.stop();
    }

    export function toggleRecording() {
        if (isRecording) {
            stopRecording();
        } else if (!isProcessing) {
            startRecording();
        }
    }
</script>

<section {id} class="mx-32 my-4 text-center">
    <button
        class="p-2 mx-32 my-2 rounded-sm {isRecording
            ? 'bg-red-500 hover:bg-red-600'
            : isProcessing
              ? 'bg-orange-800 hover:bg-orange-900'
              : 'bg-emerald-200 hover:bg-emerald-600'}"
        onclick={toggleRecording}
        disabled={isProcessing}>{buttonText}</button
    >
    <hr />
    <span>{recordingText}</span>
</section>
