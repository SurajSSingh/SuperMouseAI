<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { type RecordingStates } from "$lib/types";
    import MicRecorder from "$lib/MicRecorder.svelte";
    import { NotificationSystem } from "./notificationSystem.svelte";
    import Status from "./components/Status.svelte";

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

    let microphonePermission = $state(false);

    $effect(() => {
        const queryPermissions = async () => {
            microphonePermission =
                // @ts-ignore 'microphone' should be querable for permissions
                (await navigator.permissions.query({ name: "microphone" }))
                    .state === "granted";
        };
        queryPermissions();
    });

    async function resetPermission() {
        // await invoke("reset_permission", { origin: window.origin });
        await setupRecorder();
        const devices = await navigator.mediaDevices.enumerateDevices();
        console.log("Devices: ", devices);
    }
</script>

{#snippet PermissionRow(
    name: string,
    status: boolean,
    disabled: boolean,
    onclick: () => void,
    icon?: string,
)}
    <div>
        <h3>
            <Status
                color={status ? "success" : "error"}
                size="lg"
                class="mr-2"
            />{icon ? `${icon}: ` : ""}{name}
        </h3>
        <span class="pl-0 text-xs"
            >Permission: {status ? "Granted" : "Denied"}</span
        >
    </div>
    <Button
        color="secondary"
        size="sm"
        class="m-2 text-xs"
        {disabled}
        {onclick}
    >
        Ask {icon || name} Permission Again
    </Button>
{/snippet}

<h2 class="text-md text-center">Check Permissions</h2>
<div class="grid grid-cols-2 my-1">
    {@render PermissionRow(
        "Microphone",
        microphonePermission,
        recordingState !== "stopped",
        resetPermission,
        "🎤",
    )}
    {@render PermissionRow(
        "Notification",
        notifier.permissionGranted,
        notifier.permissionGranted,
        () => notifier.getPermissionToNotify(testNotify),
        "🔔",
    )}
</div>
