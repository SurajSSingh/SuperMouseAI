<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { type RecordingStates } from "$lib/types";
    import MicRecorder from "$lib/MicRecorder.svelte";
    import { NotificationSystem } from "./notificationSystem.svelte";

    interface PermissionsPageProps {
        setupRecorder: () => Promise<void>;
        recordingState: RecordingStates;
        notifier: NotificationSystem;
        testNotify?: boolean;
    }

    let {
        setupRecorder,
        recordingState,
        notifier,
        testNotify = false,
    }: PermissionsPageProps = $props();

    async function resetPermission() {
        // await invoke("reset_permission", { origin: window.origin });
        await setupRecorder();
        const devices = await navigator.mediaDevices.enumerateDevices();
        console.log("Devices: ", devices);
    }
</script>

<h2 class="text-md text-center">Check Permissions</h2>
<div class="grid grid-cols-2 my-1">
    <Button
        color="secondary"
        size="sm"
        class="m-2"
        disabled={recordingState !== "stopped"}
        onclick={resetPermission}
    >
        Ask Microphone (🎤) Permission Again
    </Button>
    <Button
        color="secondary"
        size="sm"
        class="m-2"
        onclick={() => notifier.getPermissionToNotify(testNotify)}
        disabled={notifier.permissionGranted}
    >
        Ask Notification (🔔) Permission Again
    </Button>
</div>
