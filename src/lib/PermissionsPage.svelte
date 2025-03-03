<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import { type RecordingStates } from "$lib/types";
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

    let explicitMicrophonePermission: PermissionState = $state(
        "denied" as PermissionState,
    );
    let notificationPermission: boolean = $state(false);

    const microphonePermission: boolean | null = $derived(
        explicitMicrophonePermission === "prompt"
            ? null
            : explicitMicrophonePermission === "granted",
    );

    $effect(() => {
        const queryPermissions = async () => {
            notificationPermission = await notifier.checkPermissionGranted();
            await updateMicrophonePermissions();
        };
        queryPermissions();
    });

    async function updateMicrophonePermissions() {
        explicitMicrophonePermission =
            // @ts-ignore 'microphone' should be querable for permissions
            (await navigator.permissions.query({ name: "microphone" })).state;
    }

    async function resetPermission() {
        let devices = await navigator.mediaDevices.enumerateDevices();
        if (devices.length > 0) {
            await updateMicrophonePermissions();
            return;
        }
        await setupRecorder();
        devices = await navigator.mediaDevices.enumerateDevices();
    }
</script>

{#snippet PermissionRow(
    name: string,
    status: boolean | null,
    disabled: boolean,
    onclick: () => void,
    icon?: string,
)}
    <div>
        <h3>
            <Status
                color={status === null
                    ? "warning"
                    : status
                      ? "success"
                      : "error"}
                size="lg"
                class="mr-2"
            />{icon ? `${icon}: ` : ""}{name}
        </h3>
        <span class="pl-0 text-xs"
            >Permission: {status === null
                ? "Prompted (User)"
                : status
                  ? "Granted"
                  : "Denied"}</span
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
        microphonePermission === true || recordingState !== "stopped",
        resetPermission,
        "🎤",
    )}
    {@render PermissionRow(
        "Notification",
        notificationPermission,
        notificationPermission,
        () => notifier.getPermissionToNotify(testNotify),
        "🔔",
    )}
</div>
