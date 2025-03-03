<script lang="ts">
    interface WhisperOptionProps {
        threads?: number;
        translate?: boolean;
        individualWordTimestamps?: boolean;
        initialPrompt?: string;
        language?: string;
        ignoredWords?: string[];
    }

    let {
        threads = $bindable(0),
        translate = $bindable(),
        individualWordTimestamps = $bindable(),
        initialPrompt = $bindable(),
        language = $bindable("en"),
        ignoredWords = $bindable(),
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
        bind:value={threads}
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
        bind:value={initialPrompt}
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
        bind:value={() => ignoredWords?.join("\n") || "",
        (words) => {
            ignoredWords = words.split("\n").map((w) => w.trim());
        }}
    ></textarea>
    <p class="fieldset-label">
        Specify words to ignore (define each on new line)
    </p>
</fieldset>
