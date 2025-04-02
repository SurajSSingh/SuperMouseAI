<script lang="ts">
    import { commands } from "$lib/bindings";
    import { configStore } from "$lib/store.svelte";
    import CollapseableFieldSet from "./ui/CollapseableFieldSet.svelte";
    import ToggleSwitch from "./ui/ToggleSwitch.svelte";

    interface AppOptionProps {}

    let {}: AppOptionProps = $props();
</script>

<h2 class="text-md text-center">Application options.</h2>
<CollapseableFieldSet
    title="General Configuration"
    titleTag="h3"
    subtitle="Configure how the app operates"
>
    <div class="mb-4">
        <ToggleSwitch
            label="Window Always on Top:"
            bind:checked={configStore.windowFloat.value}
            onchange={(e) => commands.setWindowTop(e.currentTarget.checked)}
        />
        <p class="fieldset-label">
            This will make it so the window is always on top.
        </p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Auto-Paste:"
            bind:checked={configStore.autoPaste.value}
        />
        <p class="fieldset-label">
            Immediately paste the value after transcribing (in active area
            selected by cursor).
        </p>
    </div>
    <div class="mb-4">
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
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Remove Inter-space Newlines:"
            bind:checked={configStore.interNLRemove.value}
        />
        <p class="fieldset-label">
            Will remove extra new-lines that happen inside a sentence (between
            words without puncuation).
        </p>
    </div>
</CollapseableFieldSet>
<CollapseableFieldSet
    title="Update Configuration"
    titleTag="h3"
    subtitle="Configure how app updates itself"
>
    <!-- TODO(@): Add and use config -->
    <div class="mb-4">
        <ToggleSwitch label="Notify about updates:" checked={true} />
        <p class="fieldset-label">Notify of update when they are available.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch label="Auto check for updates:" checked={true} />
        <p class="fieldset-label">Periodically check for update (~5 min.)</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch label="Auto-install" checked={true} />
        <p class="fieldset-label">
            Automatically download and install update if one is available.
        </p>
    </div>
</CollapseableFieldSet>
