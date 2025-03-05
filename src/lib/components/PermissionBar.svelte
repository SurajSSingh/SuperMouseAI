<script lang="ts">
    import { NotificationSystem } from "$lib/notificationSystem.svelte";
    import { type RecordingStates } from "$lib/types";
    import PermissionButton from "$lib/components/PermissionButton.svelte";

    interface PermissionsBarProps {
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
    }: PermissionsBarProps = $props();

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

<div>
    <!-- <h2 class="text-center text-xl">Permissions</h2> -->
    <div
        class="grid grid-cols-1 gap-1 md:gap-2 xl:gap-4 sm:grid-cols-2 lg:grid-cols-3 justify-items-center place-items-center"
    >
        <PermissionButton
            name="Microphone"
            icon="🎤"
            status={microphonePermission}
            onclick={resetPermission}
            showName={false}
        />
        <PermissionButton
            name="Notification"
            icon="🔔"
            status={notificationPermission}
            onclick={() => notifier.getPermissionToNotify(testNotify)}
            showName={false}
        />
    </div>
</div>
