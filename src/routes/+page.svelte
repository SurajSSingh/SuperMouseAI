<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    MediaRecorder as ExtendedMediaRecorder,
    register as registerConnection,
    deregister as deregisterConnection,
    type IMediaRecorder,
  } from "extendable-media-recorder";
  import { connect } from "extendable-media-recorder-wav-encoder";
  import {
    register as registerShortcut,
    unregisterAll as unregisterAllShortcuts,
  } from "@tauri-apps/plugin-global-shortcut";

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
  // FIXME: Type should not be any
  let audioRecorder: IMediaRecorder | null = $state(null);
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

  let wavRecorderConnection: MessagePort | undefined;

  // Effects
  $effect(() => {
    // On-Mount
    const setup = async () => {
      wavRecorderConnection = await connect();
      await registerConnection(wavRecorderConnection);
      await registerShortcut("CommandOrControl+Shift+R", (event) => {
        if (event.state === "Released") {
          toggleRecord();
        }
      });
    };
    // Get user permission to use mircophone
    setup().then(() => getPermission());

    return async () => {
      // Clean-up code
      if (currentURL) {
        window.URL.revokeObjectURL(currentURL);
      }
      audioElement.src = "";
      audioRecorder = null;
      if (wavRecorderConnection) deregisterConnection(wavRecorderConnection);
      unregisterAllShortcuts();
    };
  });

  async function resetPermission() {
    // await invoke("reset_permission", { origin: window.origin });
    await getPermission();
    const devices = await navigator.mediaDevices.enumerateDevices();
    console.log("Devices: ", devices);
  }

  async function getPermission() {
    if (navigator) {
      try {
        const audioStream = await navigator.mediaDevices.getUserMedia({
          audio: {
            autoGainControl: false,
            noiseSuppression: true,
            echoCancellation: true,
          },
          video: false,
        });
        console.log(audioStream);
        audioRecorder = new ExtendedMediaRecorder(audioStream, {
          mimeType: "audio/wav",
        });
        // audioRecorder = new MediaRecorder(audioStream);
        console.log(audioRecorder);
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
      } catch (error) {
        alert(`Ran into an error: ${error}`);
      }
    } else {
      alert("Window has no navigator");
    }
  }

  /**
   * Convert a blob to array of bytes
   */
  async function blobToBytes(blob: Blob): Promise<Uint8Array> {
    if (blob.bytes) return blob.bytes();
    // Fallback to making bytes from array buffer
    return new Uint8Array(await blob.arrayBuffer());
  }

  function startRecording() {
    recordingState = "recording";
    blobChunks = [];
    // Remove old URL to clear old audio cache
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
    console.log(blobChunks.length);
    const blob =
      blobChunks.length === 1
        ? blobChunks[0]!
        : new Blob(blobChunks, { type: "audio/wav" });
    currentURL = window.URL.createObjectURL(blob);
    audioElement.src = currentURL;
    try {
      transcribedOutput = await invoke("transcribe", {
        audioData: await blobToBytes(blob),
      });
    } catch (error) {
      alert(`An error occured while transcribing: ${error}`);
    }
    copyToClipboard();
    recordingState = "stopped";
  }

  function toggleRecord() {
    // When no recorder or is processing, do nothing
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

  function copyToClipboard() {
    if (navigator?.clipboard) {
      navigator.clipboard.writeText(transcribedOutput);
    }
  }
</script>

<main class="container">
  <h1 class="text-3xl text-center">SuperMouse AI</h1>
  <div class="flex flex-col place-content-center">
    <button
      class="p-2 mx-32 my-2 text-sm bg-amber-500 rounded-sm hover:bg-amber-600"
      onclick={resetPermission}
      disabled={audioRecorder !== null}>Ask Permission Agin</button
    >
    <section id="audio-holder" class="mx-32 my-4 text-center">
      <button
        class="p-2 mx-32 my-2 rounded-sm {isRecording
          ? 'bg-red-500 hover:bg-red-600'
          : isProcessing
            ? 'bg-orange-800 hover:bg-orange-900'
            : 'bg-emerald-200 hover:bg-emerald-600'}"
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
      <h2 class="text-lg text-center">Audio Preview</h2>
      <audio class="w-full" controls bind:this={audioElement}></audio>
    </section>
    <div class="mx-32 my-4 text-center rounded-md border-4 min-h-32">
      {#if transcribedOutput}
        <output>
          {transcribedOutput}
        </output>
      {:else}
        <em> This is where output goes </em>
      {/if}
    </div>
    <button
      class="p-2 mx-32 my-2 rounded-sm bg-slate-100 hover:bg-slate-200"
      onclick={copyToClipboard}
      disabled={isProcessing || isRecording}>Copy to Clipboard</button
    >
  </div>
</main>
