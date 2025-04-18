<script lang="ts">
    import { configStore } from "$lib/store.svelte";
    import CollapseableFieldSet from "./ui/CollapseableFieldSet.svelte";
    import ToggleSwitch from "./ui/ToggleSwitch.svelte";
    import WhisperModelManager from "./WhisperModelManager.svelte";

    interface WhisperOptionProps {
        translate?: boolean;
        individualWordTimestamps?: boolean;
        language?: string;
    }

    let {
        translate = $bindable(),
        individualWordTimestamps = $bindable(),
        language = $bindable("en"),
    }: WhisperOptionProps = $props();
</script>

<h2 class="text-md text-center">Configuration options for Whisper Model.</h2>
<CollapseableFieldSet
    title="Model Select"
    titleTag="h3"
    subtitle="Select the Whisper model to use for transcription"
>
    <div class="mb-4">
        <WhisperModelManager />
    </div>
</CollapseableFieldSet>
<CollapseableFieldSet
    title="Model Configuration"
    titleTag="h3"
    subtitle="Configure how Whisper model works"
>
    <div class="mb-2">
        <ToggleSwitch
            label="Enable GPU Acceleration"
            bind:checked={configStore.useGPU.value}
        />
        <p class="fieldset-label">
            Attempt to use GPU for transcription, otherwise falls back to CPU
            (uses multi-threading when possible)
        </p>
    </div>
    <div class="mb-4">
        <label for="threads-option" class=" font-semibold"
            ># of CPU threads to use?</label
        >
        <input
            type="number"
            name="threads"
            id="threads-option"
            min="0"
            bind:value={configStore.threads.value}
            class="p-1 rounded-sm border-1"
        />
        <p class="fieldset-label">
            0 = Use all, otherwise, limited to the provided number
        </p>
    </div>
    <div class="mb-4">
        <label for="patience-option" class=" font-semibold"
            >Patience Value</label
        >
        <input
            type="number"
            name="patience"
            id="patience-option"
            min="0"
            max="10"
            step="0.25"
            bind:value={configStore.patience.value}
            class="p-1 rounded-sm border-1 w-1/2"
        />
        <p class="fieldset-label">
            How long to wait before considering the current word. Longer spans
            give more context, which can make it more accurate, but takes longer
            to process and can hallucinate on silence.
        </p>
    </div>
    <div class="mb-4">
        <label for="prompt-option" class=" font-semibold">Initial Prompt</label>
        <input
            type="text"
            name="prompt"
            id="promt-option"
            placeholder="Using default prompt."
            bind:value={configStore.initialPrompt.value}
            class="p-1 rounded-sm border-1 w-full"
        />
        <p class="fieldset-label">Can use to define style or fix spelling.</p>
    </div>
</CollapseableFieldSet>
<CollapseableFieldSet
    title="Post-Processing Configuration"
    titleTag="h3"
    subtitle="Configure how the text is processed after finishing transcription."
>
    <div class="mb-4">
        <label for="ignored-words-option" class=" font-semibold"
            >Ignore Words</label
        >
        <textarea
            name="ignored-words"
            id="ignored-words-option"
            placeholder="List of words to ignore."
            class="p-1 rounded-sm border-1 w-full"
            rows={5}
            bind:value={configStore.ignoredWords.value}
        ></textarea>
        <p class="fieldset-label">
            Specify words to ignore (define each on new line)
        </p>
    </div>
</CollapseableFieldSet>
