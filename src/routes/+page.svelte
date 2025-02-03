<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  let blobChunks: Blob[] = [];
  let audioElement: HTMLAudioElement;

  // State
  let recordingState: null | "stopped" | "recording" | "processing" =
    $state(null);
  let audioRecorder: MediaRecorder | null = $state(null);
  let audioData = $state([]);
  let currentURL: string | undefined = $state();
  let transcribedOutput = $state("");

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
    isRecording ? "RECORDING" : isProcessing ? "Processing File" : "Inactive",
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

    return () => {
      // Clean-up code
      if (currentURL) {
        window.URL.revokeObjectURL(currentURL);
      }
      audioElement.src = "";
      audioRecorder = null;
    };
  });

  async function blobToBytes(blob: Blob): Promise<Uint8Array> {
    return await blob.bytes();
  }

  function startRecording() {
    recordingState = "recording";
    blobChunks = [];
    if (currentURL) {
      window.URL.revokeObjectURL(currentURL);
    }
    // audioRecorder?.start();
  }

  function stopRecording() {
    // audioRecorder?.stop();
    processData();
  }

  async function processData() {
    recordingState = "processing";
    const blob = new Blob(blobChunks, { type: "audio/mp3;" });
    currentURL = window.URL.createObjectURL(blob);
    audioElement.src = currentURL;
    transcribedOutput = await invoke("transcribe", {
      audioData: await blobToBytes(blob),
    });
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
      <button
        class="p-2 mx-32 my-2 rounded-sm {isRecording
          ? 'bg-red-500'
          : isProcessing
            ? 'bg-orange-800'
            : 'bg-emerald-200'} "
        onclick={toggleRecord}
        disabled={isProcessing}>{buttonText}</button
      >
      <hr />
      <span>{recordingText}</span>
      <br />
      <!-- <span class="text-xs"
        >{audioRecorder?.mimeType || "No Recorder"}: {currentURL ??
          "No URL"}</span
      > -->
      <audio class="w-full" controls bind:this={audioElement}></audio>
    </section>
    <div class="mx-32 my-4 text-center rounded-md border-4 min-h-32">
      <output>{transcribedOutput || "This is where output goes"}</output>
    </div>
  </div>
</main>
