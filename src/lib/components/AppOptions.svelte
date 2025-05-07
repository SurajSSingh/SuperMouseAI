<script lang="ts">
    import { commands } from "$lib/bindings";
    import { configStore } from "$lib/store.svelte";
    import { emit } from "@tauri-apps/api/event";
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
    <div class="mb-4">
        <ToggleSwitch
            label="Crash Report:"
            bind:checked={
                () => configStore.enableCrashReport.value === true,
                (v) => (configStore.enableCrashReport.value = v)
            }
            indeterminate={configStore.enableCrashReport.value === null}
            disabled={true}
        />
        <p class="fieldset-label">
            Allow running telemetry for app issues and crash reports.
        </p>
    </div>
</CollapseableFieldSet>
<CollapseableFieldSet
    title="Audio Configuration"
    titleTag="h3"
    subtitle="Configure how app records and processes audio"
>
    <div class="mb-4">
        <ToggleSwitch
            label="Echo Cancellation:"
            bind:checked={configStore.echoCancellation.value}
        />
        <p class="fieldset-label">Cancel (most) echo coming from speaker.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Noise Suppression:"
            bind:checked={configStore.noiseSuppression.value}
        />
        <p class="fieldset-label">Runs basic noise suppression.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Auto Gain Control:"
            bind:checked={configStore.autoGainControl.value}
        />
        <p class="fieldset-label">Dynamically change microphone volume.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Denoise Audio:"
            bind:checked={configStore.denoise_audio.value}
        />
        <p class="fieldset-label">Run audio de-noising before transcribing.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Normalize Audio:"
            bind:checked={configStore.normalize_result.value}
        />
        <p class="fieldset-label">
            Boost low volume audio during processing if needed.
        </p>
    </div>
</CollapseableFieldSet>
<CollapseableFieldSet
    title="Update Configuration"
    titleTag="h3"
    subtitle="Configure how app updates itself"
>
    <div class="mb-4">
        <ToggleSwitch
            label="Notify about updates:"
            bind:checked={configStore.notifyOfUpdates.value}
        />
        <p class="fieldset-label">Notify of update when they are available.</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Auto check for updates:"
            bind:checked={configStore.autoCheckForUpdates.value}
            onchange={(e) =>
                // FIXME: This hack is to send an event to updater,
                //         should use proper JS event dispatching
                emit("autoCheckUpdate", e.currentTarget.checked)}
        />
        <p class="fieldset-label">Periodically check for update (~5 min.)</p>
    </div>
    <div class="mb-4">
        <ToggleSwitch
            label="Auto-install"
            bind:checked={configStore.autoApplyUpdates.value}
        />
        <p class="fieldset-label">
            Automatically download and install update if one is available.
        </p>
    </div>
</CollapseableFieldSet>
