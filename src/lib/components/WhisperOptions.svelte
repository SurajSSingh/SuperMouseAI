<script lang="ts">
    import { configStore } from "$lib/store.svelte";

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
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <legend class="fieldset-legend"># of CPU threads to use?</legend>
    <input
        type="number"
        name="threads"
        id="threads-option"
        min="0"
        bind:value={configStore.threads.value}
        class="p-1 rounded-sm border-1"
    />
    <p class="fieldset-label">0 = Use all, otherwise, limited to number</p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <legend class="fieldset-legend">Initial Prompt</legend>
    <input
        type="text"
        name="prompt"
        id="promt-option"
        placeholder="Using default prompt."
        bind:value={configStore.initialPrompt.value}
        class="p-1 rounded-sm border-1 w-full"
    />
    <p class="fieldset-label">Can use to define style or fix spelling.</p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <legend class="fieldset-legend">Ignore Words</legend>
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
</fieldset>
