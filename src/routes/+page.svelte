<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  // State
  let recordingState: null | "stopped" | "recording" | "processing" =
    $state(null);
  let audioRecorder: MediaRecorder | null = $state(null);
  let audioData = $state([]);
  let blobChunks: Blob[] = [];
  let audioElement: HTMLAudioElement;

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

  // Effects
  $effect(() => {
    // On-Mount
    // Get user permission to use mircophone
    const getPermission = async () => {
      if (navigator) {
        const audioStream = await navigator.mediaDevices.getUserMedia({
          audio: true,
          video: false,
        });
        audioRecorder = new MediaRecorder(audioStream);
        audioRecorder.ondataavailable = (blobEvent) => {
          blobChunks.push(blobEvent.data);
        };
        audioRecorder.state;
        audioRecorder.onstart = (_e) => {
          console.log("Starting...");
          startRecording();
        };
        audioRecorder.onstop = (_e) => {
          console.log("Stopping...");
          stopRecording();
        };
      } else {
        alert("No navigator");
      }
    };
    getPermission();
  });

  function startRecording() {
    recordingState = "recording";
    // audioRecorder?.start();
  }

  function stopRecording() {
    // audioRecorder?.stop();
    processData();
  }

  async function processData() {
    recordingState = "processing";

    for await (const blob of blobChunks) {
      // const data = new Blob(blob, {'type' : 'audio/ogg; codecs=opus' });
      audioElement.src = window.URL.createObjectURL(blob);
      break;
    }
    blobChunks = [];
    recordingState = "stopped";
  }

  function toggleRecord() {
    // When no record or processing, do nothing
    if (!audioRecorder || isProcessing) {
      return;
    }
    // Not recording -> start recording
    if (!isRecording) {
      audioRecorder.start();
    }
    // Recording -> stop recording
    else {
      audioRecorder.stop();
    }
  }
</script>

<main class="container">
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <section id="audio-holder" class="mx-32 my-4 text-center">
      <audio class="w-full" controls bind:this={audioElement}></audio>
      <button
        class="bg-yellow-500 mx-64 my-2 p-2 rounded-sm"
        onclick={toggleRecord}
        disabled={isProcessing}>{buttonText}</button
      >
    </section>
    <div class="mx-32 my-4 text-center border-4 rounded-md min-h-32">
      <output>This is where output goes</output>
    </div>
  </div>
</main>
