<script lang="ts">
  import { Toaster } from "$lib/components/ui/sonner";
  import { type RecordingStates } from "$lib/types";
  import MicRecorder from "$lib/MicRecorder.svelte";
  import AudioTranscriber from "$lib/AudioTranscriber.svelte";
  import { NotificationSystem } from "$lib/notificationSystem.svelte";
  import ShortcutSettings from "$lib/ShortcutSettings.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import WhisperOptions from "$lib/WhisperOptions.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import ThemeDropdown from "$lib/components/ThemeDropdown.svelte";
  import Tab from "$lib/components/Tab.svelte";
  import PermissionsPage from "$lib/PermissionsPage.svelte";
  import CustomShortcut from "$lib/components/CustomShortcut.svelte";

  const THEMES = [
    {
      value: "system",
      label: "System",
      isDefault: true,
      kind: "system" as const,
    },
    { value: "light", label: "Light", kind: "light" as const },
    { value: "dark", label: "Dark", kind: "dark" as const },
  ];

  // Component Bindings
  let micRecorder: MicRecorder;
  let audioTranscriber: AudioTranscriber;
  // State
  let recordingState: RecordingStates = $state("stopped");
  let transcribedOutput = $state("");
  let enableSound = $state(true);
  let testNotify = $state(true);
  let threads = $state(0);
  let initialPrompt = $state("");
  let theme: "system" | "light" | "dark" = $state("system");
  let ignoredWords = $state(["[BLANK_AUDIO]", "[NO_AUDIO]", "[SILENCE]"]);

  // Inner Variables
  const notifier = new NotificationSystem(
    //v: This is required to ignore the
    //   state_referenced_locally warning
    (() => enableSound)(),
    (() => testNotify)(),
  );

  // Helper Functions
  function copyToClipboard() {
    writeText(transcribedOutput);
    notifier.showNotification("Copied to clipboard!", "", "");
  }

  $inspect(recordingState);

  function transcribe(chunks?: Blob[]) {
    recordingState = "processing";
    // Force a microtask to allow rendering before transcribing,
    // fixes issue with "processing" state not updating during processing
    new Promise((resolve) => requestAnimationFrame(resolve)).finally(() => {
      audioTranscriber.processData(
        chunks,
        threads > 0 ? threads : undefined,
        initialPrompt || undefined,
        ignoredWords,
      );
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
    transcribe(chunks);
  }
  function onFinishProcessing() {
    recordingState = "stopped";
    notifier.showNotification("Transcription Finished!", "", "finish");
    copyToClipboard();
  }
  function onError(err: string) {
    alert(err);
  }
</script>

<main class="">
  <Toaster position="top-right" richColors closeButton {theme} />
  <ThemeDropdown themes={THEMES} bind:current={theme} class="fixed top-0" />
  <div class="mx-2 sm:mx-24 md:mx-32">
    <h1 class="text-3xl text-center pt-12 sm:pt-0">SuperMouse AI</h1>
    <div class="flex flex-col place-content-center">
      <section class="tabs tabs-lift">
        <Button
          color="destructive"
          variant="ghost"
          shape="circle"
          onclick={() =>
            document.getElementsByName("tabs").forEach(
              (tab) =>
                // @ts-ignore: Every tab is a radio input
                (tab.checked = false),
            )}>X</Button
        >
        <Tab
          value="tabs"
          label="Permissions"
          checked
          class="bg-base-100 border-base-300 p-6"
        >
          <PermissionsPage
            setupRecorder={() => micRecorder.setupRecorder()}
            {recordingState}
            {notifier}
            {testNotify}
          />
        </Tab>
        <Tab
          value="tabs"
          label="Settings"
          class="bg-base-100 border-base-300 p-6"
        >
          <CustomShortcut
            onToggleShortcutEvent={(e) => {
              if (e.state === "Pressed") {
                micRecorder?.toggleRecording();
              }
            }}
            {notifier}
          />
        </Tab>
        <Tab
          value="tabs"
          label="Configuration"
          class="bg-base-100 border-base-300 p-6"
        >
          <WhisperOptions bind:threads bind:initialPrompt bind:ignoredWords />
        </Tab>
      </section>
      <MicRecorder
        bind:this={micRecorder}
        {recordingState}
        {onRecordingStart}
        {onRecordingEnd}
        {onError}
      />
      <div class="grid grid-cols-2 my-1">
        <Button
          color={recordingState === "processing" ? "neutral" : "warning"}
          size="sm"
          class="m-2"
          onclick={() => {
            notifier.showNotification("Re-transcribing data.", "", "stop");
            transcribe();
          }}
          disabled={recordingState !== "stopped"}>(✏️) Re-transcribe</Button
        >
        <Button
          color="info"
          size="sm"
          class="m-2"
          onclick={copyToClipboard}
          disabled={recordingState !== "stopped"}>(📋) Copy to Clipboard</Button
        >
      </div>
      <AudioTranscriber
        bind:this={audioTranscriber}
        bind:transcribedOutput
        {onFinishProcessing}
        {onError}
      />
    </div>
  </div>
</main>
