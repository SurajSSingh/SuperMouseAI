<script lang="ts">
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

<details class="p-2 mx-32 my-1 text-sm rounded-md border-2">
    <summary class="p-2 bg-amber-50 hover:bg-amber-200 rounded-md"
        >Advanced Options</summary
    >
    <h2 class="text-md text-center">Configure options for Whisper here.</h2>
    <hr />
    <div class="my-4 p-2 rounded-sm bg-slate-200">
        <label for="threads" class="px-2 font-bold"
            ># of CPU threads to use?</label
        >
        <input
            type="number"
            name="threads"
            id="threads-option"
            min="0"
            bind:value={threads}
            class="p-1 rounded-sm border-1"
        />
        <p class="ml-48 text-sm">0 = Use all, otherwise, limited to number</p>
    </div>
    <div class="my-4 p-2 rounded-sm bg-slate-200">
        <label for="prompt" class="px-2 font-bold">Initial Prompt</label>
        <input
            type="text"
            name="prompt"
            id="promt-option"
            bind:value={initialPrompt}
            class="p-1 rounded-sm border-1 w-full"
        />
        <p class="ml-2 text-sm">Can use to define style or fix spelling.</p>
    </div>
</details>
