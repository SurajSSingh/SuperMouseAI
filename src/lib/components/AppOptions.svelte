<script lang="ts">
    import { commands } from "$lib/bindings";
    import { configStore } from "$lib/store.svelte";
    import ToggleSwitch from "./ui/ToggleSwitch.svelte";

    interface AppOptionProps {}

    let {}: AppOptionProps = $props();
</script>

<h2 class="text-md text-center">Application options.</h2>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <ToggleSwitch
        label="Window Always on Top:"
        bind:checked={configStore.windowFloat.value}
        onchange={(e) => commands.setWindowTop(e.currentTarget.checked)}
    />
    <p class="fieldset-label">
        This will make it so the window is always on top.
    </p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <ToggleSwitch
        label="Auto-Paste:"
        bind:checked={configStore.autoPaste.value}
    />
    <p class="fieldset-label">
        Immediately paste the value after transcribing (in active area selected
        by cursor).
    </p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <ToggleSwitch
        label="Use Ctrl+V for auto-paste:"
        bind:checked={configStore.pasteViaKeys.value}
    />
    <p class="text-base-content/60">
        When toggled on, auto-paste with <kbd class="kbd kbd-sm text-accent"
            >Ctrl + V</kbd
        >. When off, it will write the text (NOTE: uses
        <kbd class="kbd kbd-sm text-accent">Enter</kbd> for newlines)
    </p>
</fieldset>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <ToggleSwitch
        label="Remove Inter-space Newlines:"
        bind:checked={configStore.interNLRemove.value}
    />
    <p class="fieldset-label">
        Will remove extra new-lines that happen inside a sentence (between words
        without puncuation).
    </p>
</fieldset>
