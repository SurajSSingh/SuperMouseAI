<script lang="ts">
  import { type RecordingStates } from "$lib/types";
  import MicRecorder from "$lib/MicRecorder.svelte";
  import AudioTranscriber from "$lib/AudioTranscriber.svelte";
  import { NotificationSystem } from "$lib/notificationSystem.svelte";
  import ShortcutSettings from "$lib/ShortcutSettings.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import WhisperOptions from "$lib/WhisperOptions.svelte";

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
  let threads = $state(0);
  let initialPrompt = $state("");

  // Inner Variables
  const notifier = new NotificationSystem(enableSound, testNotify);

  // Helper Functions
  function copyToClipboard() {
    writeText(transcribedOutput);
    notifier.showNotification("Copied to clipboard!", "", "");
  }

  function transcribe(chunks?: Blob[]) {
    recordingState = "processing";
    audioTranscriber.processData(
      chunks,
      threads > 0 ? threads : undefined,
      initialPrompt || undefined,
    );
  }
  // Callback functions
  function onRecordingStart() {
    recordingState = "recording";
    notifier.showNotification("Recording Started!", "", "start");
  }
  function onRecordingEnd(chunks: Blob[]) {
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
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <div class="grid grid-cols-2 mx-32 my-1">
      <button
        class="p-2 m-2 text-sm bg-amber-500 rounded-sm hover:bg-amber-600"
        onclick={resetPermission}
      >
        Ask Permission Again
      </button>
      <button
        class="p-2 m-2 text-sm bg-amber-500 rounded-sm hover:bg-amber-600"
        onclick={() => notifier.getPermissionToNotify(testNotify)}
        disabled={notifier.permissionGranted}
        >Ask Notification Permission Again</button
      >
    </div>
    <WhisperOptions bind:threads bind:initialPrompt />
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
    <div class="grid grid-cols-2 mx-32 my-1">
      <button
        class="p-2 m-2 rounded-sm bg-green-100 hover:bg-green-200"
        onclick={() => {
          notifier.showNotification("Re-transcribing data.", "", "stop");
          transcribe();
        }}
        disabled={recordingState !== "stopped"}
      >
        Re-transcribe</button
      >
      <button
        class="p-2 m-2 rounded-sm bg-green-100 hover:bg-green-200"
        onclick={copyToClipboard}
        disabled={recordingState !== "stopped"}
      >
        Copy to Clipboard</button
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
