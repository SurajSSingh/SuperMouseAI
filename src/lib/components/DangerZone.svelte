<script lang="ts">
    import { notifier } from "$lib/notificationSystem.svelte";
    import { configStore } from "$lib/store.svelte";
    import { Button } from "./ui/button";

    interface DangerZoneProps {
        isDialogOpen?: boolean;
    }

    let { isDialogOpen = $bindable(false) }: DangerZoneProps = $props();
</script>

<div
    class="tooltip tooltip-right mb-1"
    data-tip="Click to clear all transcripts."
>
    <Button
        color="destructive"
        onclick={() => {
            isDialogOpen = false;
            notifier.confirmAction(
                "You will clear all transcriptions!",
                () => configStore.clearTranscripts(),
                () => {},
                "Are you sure?",
            );
        }}>Delete All Transcripts</Button
    >
</div>
<br />
<div
    class="tooltip tooltip-right mb-1"
    data-tip="Click to delete all configuration data."
>
    <Button
        color="destructive"
        onclick={() => {
            isDialogOpen = false;
            notifier.confirmAction(
                "You will clear all transcriptions alongside any customizations you have made. This will take effect AFTER closing the app.",
                () => configStore.clearData(),
                () => {},
                "Are you sure?",
            );
        }}>Clear App Data</Button
    >
</div>
