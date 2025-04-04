<script lang="ts">
    import { Toaster } from "$lib/components/ui/sonner";
    import { type RecordingStates } from "$lib/types";
    import MicRecorder from "$lib/components/MicRecorder.svelte";
    import AudioTranscriber from "$lib/components/AudioTranscriber.svelte";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";
    import Button from "$lib/components/ui/button/button.svelte";
    import ThemeDropdown from "$lib/components/ui/ThemeDropdown.svelte";
    import CustomShortcut from "$lib/components/CustomShortcut.svelte";
    import MenuScreen from "$lib/components/MenuScreen.svelte";
    import PermissionBar from "$lib/components/PermissionBar.svelte";
    import { configStore } from "$lib/store.svelte";
    import { commands } from "$lib/bindings";
    import { exit } from "@tauri-apps/plugin-process";
    import { error, info, warn } from "@tauri-apps/plugin-log";
    import { getVersion } from "@tauri-apps/api/app";
    import { emitTo } from "@tauri-apps/api/event";
    import UpdateChecker from "$lib/components/UpdateChecker.svelte";
    import ModelDropdown from "$lib/components/ui/ModelDropdown.svelte";

    // Component Bindings
    let micRecorder: MicRecorder = $state() as MicRecorder;
    let audioTranscriber: AudioTranscriber = $state() as AudioTranscriber;
    // State
    let recordingState: RecordingStates = $state("stopped");
    let hasRecorded = $state(false);
    let menuOpen = $state(false);
    let appVersion = $state("unknown");
    let acceptTelemetry: boolean = $state(false);

    // Helper Functions
    function copyToClipboard() {
        writeText(configStore.currentTranscript);
        notifier.showInfo("Copied to clipboard!", "", "");
        notifier.showNotification("Copied to clipboard!", "", "");
    }

    function transcribe(chunks?: Blob[]) {
        recordingState = "processing";
        emitTo("overlay", "stateUpdate", { state: recordingState });
        // Force a microtask to allow rendering before transcribing,
        // fixes issue with "processing" state not updating during processing
        new Promise((resolve) => requestAnimationFrame(resolve)).finally(() => {
            audioTranscriber.processData(chunks);
        });
    }

    // Callback functions
    function onRecordingStart() {
        recordingState = "recording";
        emitTo("overlay", "stateUpdate", { state: recordingState });
        notifier.showNotification("Recording Started!", "", "start");
    }
    function onRecordingEnd(chunks: Blob[]) {
        recordingState = "processing";
        emitTo("overlay", "stateUpdate", { state: recordingState });
        notifier.showNotification("Recording Stopped!", "", "stop");
        hasRecorded = true;
        transcribe(chunks);
    }
    function onFinishProcessing() {
        recordingState = "stopped";
        emitTo("overlay", "stateUpdate", { state: recordingState });
        notifier.showNotification("Transcription Finished!", "", "finish");
        copyToClipboard();
        if (configStore.autoPaste.value) {
            if (configStore.pasteViaKeys.value) {
                commands.pasteText().catch((err) => notifier.showError(err));
            } else {
                commands
                    .writeText(configStore.currentTranscript)
                    .catch((err) => notifier.showError(err));
            }
        }
    }
    function onError(err: string) {
        error(`Got error: ${error}`);
        notifier.showError(err);
    }

    // Top-level clean-up ONLY (for store)
    $effect(() => {
        const showNotice = async () => {
            info("Show telemetry notice");
            const accepted = await notifier.showDialog(
                "ask",
                "We collect basic error and crash reports by default using Sentry. We DO NOT collect your private data (such as audio data or transcripts), only information related to OS (like which GPU you use) and actions that lead to the app showing an error or crashing. This cannot be turned off for pre-release builds of this app. By continuing, you agree to the terms.",
                {
                    kind: "warning",
                    title: "Telemetry Notice",
                    okLabel: "I Agree",
                    cancelLabel: "Quit App",
                },
            );
            if (accepted) {
                info("User has accepted to use the app.");
                acceptTelemetry = true;
            } else {
                // Exit immediately
                exit(0);
            }
        };
        getVersion().then(
            (version) =>
                (appVersion = version.startsWith("v")
                    ? version
                    : `v${version}`),
        );
        showNotice();
        configStore.waitForStoreLoaded().then(() => {
            info("Set window float value");
            commands
                .setWindowTop(configStore.windowFloat.value)
                .catch((err) => {
                    // Warning, not error, because user can manually set it
                    warn(`Window float value could not be set: ${err}`);
                    notifier.showToast(
                        "Could not set window float value automatically",
                        "",
                        "warn",
                        "",
                        6_000,
                    );
                });
        });
        return () => {
            configStore.cleanup();
        };
    });
</script>

<main class="">
    <MenuScreen bind:open={menuOpen} />
    <Toaster
        position="top-center"
        richColors
        closeButton
        theme={configStore.theme.value}
    />
    <ThemeDropdown
        class="fixed top-0 right-0"
        listClass="p-1 w-full"
        direction="bottom"
    />
    <div class="mx-2 sm:mx-24 md:mx-32">
        <h1 class="text-3xl text-center pt-12 sm:pt-0">
            SuperMouse AI ({appVersion})
        </h1>
        {#if acceptTelemetry}
            <div class="flex flex-col place-content-center p-1">
                <UpdateChecker class="mx-8" />
            </div>
            <PermissionBar
                setupRecorder={() => micRecorder.setupRecorder()}
                {recordingState}
            />
            <div class="flex flex-col place-content-center">
                <div class="place-self-stretch">
                    <div class="mx-2 sm:mx-16">
                        <ModelDropdown class="w-full" />
                    </div>
                </div>
                <MicRecorder
                    bind:this={micRecorder}
                    {recordingState}
                    {onRecordingStart}
                    {onRecordingEnd}
                    {onError}
                />
                <CustomShortcut
                    class="w-3/4 mb-2"
                    onToggleShortcutEvent={(e) => {
                        if (e.state === "Pressed") {
                            micRecorder?.toggleRecording();
                        }
                    }}
                />
                <hr />
                <div class="grid grid-cols-2 my-1">
                    <Button
                        color={recordingState === "processing"
                            ? "neutral"
                            : "warning"}
                        size="sm"
                        class="m-2"
                        onclick={() => {
                            notifier.showNotification(
                                "Re-transcribing data.",
                                "",
                                "stop",
                            );
                            transcribe();
                        }}
                        disabled={!hasRecorded || recordingState !== "stopped"}
                        >(✏️) Re-transcribe</Button
                    >
                    <Button
                        color="info"
                        size="sm"
                        class="m-2"
                        onclick={copyToClipboard}
                        disabled={!configStore.currentTranscript ||
                            recordingState !== "stopped"}
                        >(📋) Copy to Clipboard</Button
                    >
                </div>
                <AudioTranscriber
                    bind:this={audioTranscriber}
                    {onFinishProcessing}
                    {onError}
                />
            </div>
        {/if}
    </div>
</main>
