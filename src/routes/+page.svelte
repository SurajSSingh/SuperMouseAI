<script lang="ts">
  import { Toaster } from "$lib/components/ui/sonner";
  import { type RecordingStates } from "$lib/types";
  import MicRecorder from "$lib/components/MicRecorder.svelte";
  import AudioTranscriber from "$lib/components/AudioTranscriber.svelte";
  import { NotificationSystem } from "$lib/notificationSystem.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import WhisperOptions from "$lib/components/WhisperOptions.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import Tab from "$lib/components/ui/Tab.svelte";
  import ThemeDropdown from "$lib/components/ui/ThemeDropdown.svelte";
  import PermissionsPage from "$lib/components/PermissionsPage.svelte";
  import CustomShortcut from "$lib/components/CustomShortcut.svelte";
  import MenuScreen from "$lib/components/MenuScreen.svelte";
  import PermissionBar from "$lib/components/PermissionBar.svelte";

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
    // | This is required to ignore the
    // | state_referenced_locally warning
    // V
    (() => enableSound)(),
    (() => testNotify)(),
  );

  // Helper Functions
  function copyToClipboard() {
    writeText(transcribedOutput);
    notifier.showInfo("Copied to clipboard!", "", "");
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
  <MenuScreen>
    <section class="tabs tabs-lift w-full">
      <Tab
        value="tabs"
        label="Configuration"
        class="bg-base-100 border-base-300 p-6"
        checked
      >
        <div class="h-60 overflow-auto pr-6">
          <WhisperOptions bind:threads bind:initialPrompt bind:ignoredWords />
        </div>
      </Tab>
    </section>
  </MenuScreen>
  <Toaster position="top-center" richColors closeButton {theme} />
  <ThemeDropdown
    themes={THEMES}
    bind:current={theme}
    class="fixed top-0 right-0"
    listClass="p-1 w-full"
    direction="bottom"
  />
  <div class="mx-2 sm:mx-24 md:mx-32">
    <h1 class="text-3xl text-center pt-12 sm:pt-0">SuperMouse AI</h1>
    <PermissionBar
      setupRecorder={() => micRecorder.setupRecorder()}
      {recordingState}
      {notifier}
      {testNotify}
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
        {notifier}
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
          disabled={recordingState !== "stopped"}>(✏️) Re-transcribe</Button
        >
        <Button
          color="info"
          size="sm"
          class="m-2"
          onclick={copyToClipboard}
          disabled={!transcribedOutput || recordingState !== "stopped"}
          >(📋) Copy to Clipboard</Button
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
