<script lang="ts">
    import SingleCollapsable from "./components/SingleCollapsable.svelte";

    interface WhisperOptionProps {
        threads?: number;
        translate?: boolean;
        individualWordTimestamps?: boolean;
        initialPrompt?: string;
        language?: string;
    }

    let {
        threads = $bindable(0),
        translate = $bindable(),
        individualWordTimestamps = $bindable(),
        initialPrompt = $bindable(),
        language = $bindable("en"),
    }: WhisperOptionProps = $props();
</script>

<SingleCollapsable
    class="p-2 mx-32 my-1 text-sm rounded-md flex flex-col self-center"
    summary="Advanced Config"
>
    <h2 class="text-md text-center">
        Configuration options for Whisper Model.
    </h2>
    <fieldset
        class="fieldset w-xs bg-base-200 border border-base-300 p-4 rounded-box"
    >
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
    <fieldset
        class="fieldset w-xs bg-base-200 border border-base-300 p-4 rounded-box"
    >
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
</SingleCollapsable>
