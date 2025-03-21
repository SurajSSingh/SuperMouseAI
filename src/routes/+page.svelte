<script lang="ts">
  import { Toaster } from "$lib/components/ui/sonner";
  import { type RecordingStates } from "$lib/types";
  import MicRecorder from "$lib/components/MicRecorder.svelte";
  import AudioTranscriber from "$lib/components/AudioTranscriber.svelte";
  import { NotificationSystem, notifier } from "$lib/notificationSystem.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import WhisperOptions from "$lib/components/WhisperOptions.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import Tab from "$lib/components/ui/Tab.svelte";
  import ThemeDropdown from "$lib/components/ui/ThemeDropdown.svelte";
  import CustomShortcut from "$lib/components/CustomShortcut.svelte";
  import MenuScreen from "$lib/components/MenuScreen.svelte";
  import PermissionBar from "$lib/components/PermissionBar.svelte";
  import { configStore } from "$lib/store.svelte";
  import { commands } from "$lib/bindings";
  import DangerZone from "$lib/components/DangerZone.svelte";
  import AppOptions from "$lib/components/AppOptions.svelte";
  import { exit } from "@tauri-apps/plugin-process";
  import { error, info } from "@tauri-apps/plugin-log";

  // Component Bindings
  let micRecorder: MicRecorder;
  let audioTranscriber: AudioTranscriber;
  // State
  let recordingState: RecordingStates = $state("stopped");
  let hasRecorded = $state(false);
  let menuOpen = $state(false);

  // Helper Functions
  function copyToClipboard() {
    writeText(configStore.currentTranscript);
    notifier.showInfo("Copied to clipboard!", "", "");
    notifier.showNotification("Copied to clipboard!", "", "");
  }

  function transcribe(chunks?: Blob[]) {
    recordingState = "processing";
    // Force a microtask to allow rendering before transcribing,
    // fixes issue with "processing" state not updating during processing
    new Promise((resolve) => requestAnimationFrame(resolve)).finally(() => {
      audioTranscriber.processData(chunks);
    });
  }

  // Callback functions
  function onRecordingStart() {
    recordingState = "recording";
    notifier.showNotification("Recording Started!", "", "start");
  }
  function onRecordingEnd(chunks: Blob[]) {
    recordingState = "processing";
    notifier.showNotification("Recording Stopped!", "", "stop");
    hasRecorded = true;
    transcribe(chunks);
  }
  function onFinishProcessing() {
    recordingState = "stopped";
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
    notifier.confirmAction(
      "We collect basic error and crash reports by default using Sentry. We DO NOT collect your private data (such as audio data or transcripts), only information related to OS (like which GPU you use) and actions that lead to the app showing an error or crashing. This cannot be turned off for pre-release builds of this app. By continuing, you agree to the terms.",
      () => {
        info("User has explicitly accepted to use the app.");
      },
      () => {
        // Exit immediately
        exit(0);
      },
      "Telemetry notice",
      "I Agree",
      "Quit App",
    );
    return () => {
      configStore.cleanup();
    };
  });
</script>

<main class="">
  <MenuScreen bind:open={menuOpen}>
    <section class="tabs tabs-lift w-full">
      <Tab
        value="tabs"
        label="Configuration"
        class="bg-base-100 border-base-300 p-6"
        inputClass="not-checked:bg-neutral not-checked:text-accent-content hover:text-primary border-base-300"
        checked
      >
        <div class="h-60 overflow-auto pr-6">
          <WhisperOptions />
        </div>
      </Tab>
      <Tab
        value="tabs"
        label="App Options"
        class="bg-base-100 border-base-300 p-6"
        inputClass="not-checked:bg-neutral not-checked:text-accent-content hover:text-primary border-base-300"
      >
        <div class="h-60 overflow-auto pr-6">
          <AppOptions />
        </div>
      </Tab>
      <Tab
        value="tabs"
        label="Danger Zone"
        inputClass="input-ghost p-6 hover:bg-error not-checked:bg-neutral not-checked:text-warning-content checked:input-xl checked:text-warning"
        class="bg-base-100 border-base-300 p-6"
      >
        <div class="h-60 overflow-auto pr-6">
          <DangerZone bind:isDialogOpen={menuOpen} />
        </div>
      </Tab>
    </section>
  </MenuScreen>
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
    <h1 class="text-3xl text-center pt-12 sm:pt-0">SuperMouse AI</h1>
    <PermissionBar
      setupRecorder={() => micRecorder.setupRecorder()}
      {recordingState}
    />
    <div class="flex flex-col place-content-center">
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
          color={recordingState === "processing" ? "neutral" : "warning"}
          size="sm"
          class="m-2"
          onclick={() => {
            notifier.showNotification("Re-transcribing data.", "", "stop");
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
            recordingState !== "stopped"}>(📋) Copy to Clipboard</Button
        >
      </div>
      <AudioTranscriber
        bind:this={audioTranscriber}
        {onFinishProcessing}
        {onError}
      />
    </div>
  </div>
</main>
