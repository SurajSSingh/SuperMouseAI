<script lang="ts">
    import type { RecordingStates } from "$lib/types";
    import {
        MediaRecorder as ExtendedMediaRecorder,
        register,
        deregister,
        type IMediaRecorder,
    } from "extendable-media-recorder";
    import { connect } from "extendable-media-recorder-wav-encoder";
    import Button from "$lib/components/ui/button/button.svelte";
    import Loading from "$lib/components/ui/Loading.svelte";
    import { debug, info, trace, warn } from "@tauri-apps/plugin-log";

    interface RecorderProps {
        id?: string;
        recordingState: RecordingStates;
        onRecordingStart?: () => void;
        onRecordingEnd?: (chunks: Blob[]) => void;
        onError?: (err: string) => void;
    }

    let {
        id = "audio-holder",
        recordingState = "stopped",
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
              ? "(🗣️📝): Processing Audio"
              : "(🎤⏹️): Microphone Inactive",
    );

    // Effects
    $effect(() => {
        // On-Mount
        setupRecorder();

        return async () => {
            debug(`Running clean-up code for MicRecorder component`);
            audioRecorder = null;
            audioStream = null;
            if (wavRecorderConnection) {
                deregister(wavRecorderConnection);
                trace(`Deregister WAV recorder connection`);
            }
        };
    });

    export async function setupRecorder() {
        try {
            debug(`Setting up audio recorder`);
            trace(`Creating WAV connection`);
            wavRecorderConnection = await connect();
            trace(`Registering WAV connection`);
            await register(wavRecorderConnection);
            trace(`Getting audio media stream`);
            audioStream = await navigator.mediaDevices.getUserMedia({
                audio: {
                    autoGainControl: false,
                    noiseSuppression: true,
                    echoCancellation: true,
                },
                video: false,
            });
            debug(`Finished up audio recorder setup`);
        } catch (error) {
            onError?.(`Error on setting up recorder: ${error}`);
        }
    }

    function startRecording() {
        info(`Start audio recording`);
        if (isProcessing) {
            warn(`Attempted to start audio, but is currently processing`);
            return;
        }
        if (!audioStream) {
            onError?.("Audio Stream not setup, stopping recording!");
            return;
        }
        try {
            debug(`Create audio recorder and start it`);
            // Create a new audio to start recording
            // REASON: Maybe better security as only
            // record when user want to start recording
            audioRecorder = new ExtendedMediaRecorder(audioStream, {
                mimeType: "audio/wav",
            });
            trace(`Create audio recorder with WAV mime type`);
            audioRecorder.ondataavailable = (blobEvent) => {
                trace(`Pushing audio chunk of size: ${blobEvent.data.size}`);
                audioChunks.push(blobEvent.data);
            };
            audioRecorder.onstart = (_e) => {
                debug(`Start recording audio data`);
                // Clear old recording
                audioChunks = [];
            };
            audioRecorder.onstop = (_e) => {
                debug(`Stop recording audio data`);
                // Delete audio recorder to stop listening
                audioRecorder = null;

                onRecordingEnd?.(audioChunks);
            };
            trace(`Finsh setup for all audio recorder event callbacks`);
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
        info(`Stop audio recording`);
        if (!isRecording) {
            warn(`Cannot stop recording, as app is not recording`);
            return;
        }
        if (audioRecorder === null) {
            onError?.("No Audio recorder active!");
            return;
        }
        audioRecorder.stop();
    }

    export function toggleRecording() {
        debug(`Toggling audio recording`);
        if (isRecording) {
            stopRecording();
        } else if (!isProcessing) {
            startRecording();
        }
    }
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
        disabled={isProcessing}>{buttonText}</Button
    >
    <p class="mb-2">
        {#if isProcessing}
            <Loading />
        {/if}
        {recordingText}
    </p>
</section>
