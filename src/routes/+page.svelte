<script lang="ts">
  import { type RecordingStates } from "$lib/types";
  import MicRecorder from "$lib/MicRecorder.svelte";
  import AudioTranscriber from "$lib/AudioTranscriber.svelte";
  import { NotificationSystem } from "$lib/notificationSystem.svelte";
  import ShortcutSettings from "$lib/ShortcutSettings.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  async function resetPermission() {
    // await invoke("reset_permission", { origin: window.origin });
    await micRecorder.setupRecorder();
    const devices = await navigator.mediaDevices.enumerateDevices();
    console.log("Devices: ", devices);
  }

  // Component Bindings
  let micRecorder: MicRecorder;
  let audioTranscriber: AudioTranscriber;
  // State
  let recordingState: RecordingStates = $state("stopped");
  let transcribedOutput = $state("");
  let enableSound = $state(true);
  let testNotify = $state(true);

  // Inner Variables
  const notifier = new NotificationSystem(enableSound, testNotify);

  // Helper Functions
  function copyToClipboard() {
    writeText(transcribedOutput);
  }

  // Callback functions
  function onRecordingStart() {
    recordingState = "recording";
    notifier.showNotification("Recording Started!", "", "start");
  }
  function onRecordingEnd(chunks: Blob[]) {
    recordingState = "processing";
    notifier.showNotification("Recording Stopped!", "", "stop");
    audioTranscriber.processData(chunks);
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
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <button
      class="p-2 mx-32 my-2 text-sm bg-amber-500 rounded-sm hover:bg-amber-600"
      onclick={resetPermission}
    >
      Ask Permission Again
    </button>
    <button
      class="p-2 mx-32 my-2 text-sm bg-slate-300 rounded-sm hover:bg-amber-300"
      onclick={() => notifier.getPermissionToNotify(testNotify)}
      disabled={notifier.permissionGranted}
      >Ask Notification Permission Again</button
    >
    <MicRecorder
      bind:this={micRecorder}
      {recordingState}
      {onRecordingStart}
      {onRecordingEnd}
      {onError}
    />
    <ShortcutSettings
      onToggleShortcutEvent={() => micRecorder?.toggleRecording()}
    />
    <AudioTranscriber
      bind:this={audioTranscriber}
      bind:transcribedOutput
      {onFinishProcessing}
      {onError}
    />
    <button
      class="p-2 mx-32 my-2 rounded-sm bg-slate-100 hover:bg-slate-200"
      onclick={copyToClipboard}
      disabled={recordingState !== "stopped"}
    >
      Copy to Clipboard</button
    >
  </div>
</main>
