<script lang="ts">
    import { commands } from "$lib/bindings";
    import { configStore } from "$lib/store.svelte";
    import ToggleSwitch from "./ui/ToggleSwitch.svelte";

    interface AppOptionProps {
        windowOnTop?: boolean;
    }

    let { windowOnTop = $bindable(false) }: AppOptionProps = $props();
</script>

<h2 class="text-md text-center">Application options.</h2>
<fieldset class="fieldset bg-base-200 border border-base-300 p-4 rounded-box">
    <ToggleSwitch
        label="Window Always on Top:"
        bind:checked={windowOnTop}
        onchange={(e) => commands.setWindowTop(e.currentTarget.checked)}
    />
    <p class="fieldset-label">
        This will make it so the window is always on top.
    </p>
</fieldset>
