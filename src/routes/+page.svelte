<script lang="ts">
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

  const THEMES = [
    { value: "default", label: "System", isDefault: true },
    { value: "bumblebeeLight", label: "Light" },
    { value: "bumblebeeDark", label: "Dark" },
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

<main class="container">
  <ThemeDropdown themes={THEMES} class="fixed top-0" />
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <section class="tabs tabs-lift mx-32">
      <Tab
        value="tabs"
        label="Permissions"
        checked
        class="bg-base-100 border-base-300 p-6"
      >
        <PermissionsPage
          {micRecorder}
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
        <ShortcutSettings
          onToggleShortcutEvent={() => micRecorder?.toggleRecording()}
        />
      </Tab>
      <Tab
        value="tabs"
        label="Configuration"
        class="bg-base-100 border-base-300 p-6"
      >
        <WhisperOptions bind:threads bind:initialPrompt />
      </Tab>
    </section>
    <!-- <Tabs tabs={TABS} class="mx-32">
      <TabsContent value={TABS[0].value} class="tab-content">
        <div class="grid grid-cols-2 my-1">
          <Button
            color="secondary"
            size="sm"
            class="m-2"
            disabled={recordingState !== "stopped"}
            onclick={resetPermission}
          >
            Ask Microphone (🎤) Permission Again
          </Button>
          <Button
            color="secondary"
            size="sm"
            class="m-2"
            onclick={() => notifier.getPermissionToNotify(testNotify)}
            disabled={notifier.permissionGranted}
          >
            Ask Notification (🔔) Permission Again
          </Button>
        </div>
      </TabsContent>
      <TabsContent value={TABS[1].value} class="tab-content">
        <ShortcutSettings
          onToggleShortcutEvent={() => micRecorder?.toggleRecording()}
        />
      </TabsContent>
      <TabsContent value={TABS[2].value} class="tab-content">
        <WhisperOptions bind:threads bind:initialPrompt />
      </TabsContent>
    </Tabs> -->
    <MicRecorder
      bind:this={micRecorder}
      {recordingState}
      {onRecordingStart}
      {onRecordingEnd}
      {onError}
    />
    <div class="grid grid-cols-2 mx-32 my-1">
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
</main>
