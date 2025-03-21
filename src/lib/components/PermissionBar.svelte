<script lang="ts">
    import {
        NotificationSystem,
        notifier,
    } from "$lib/notificationSystem.svelte";
    import { type RecordingStates } from "$lib/types";
    import PermissionButton from "$lib/components/PermissionButton.svelte";
    import { configStore } from "$lib/store.svelte";
    import { debug, trace } from "@tauri-apps/plugin-log";

    interface PermissionsBarProps {
        setupRecorder: () => Promise<void>;
        recordingState: RecordingStates;
        showNames?: boolean;
        showIcons?: boolean;
    }

    let {
        setupRecorder,
        recordingState,
        showNames = true,
        showIcons = true,
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
        debug(`Update permissions for Permission Bar component`);
        queryPermissions();
        trace(`Permissions allowed
        - Notifications = ${notificationPermission}
        - Mircophone = ${explicitMicrophonePermission}
        `);
    });

    async function updateMicrophonePermissions() {
        debug(`Update microphone permission`);
        explicitMicrophonePermission =
            // @ts-ignore 'microphone' should be querable for permissions
            (await navigator.permissions.query({ name: "microphone" })).state;
    }

    async function resetPermission() {
        debug(`Resetting microphone permission`);
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
            icon={showIcons ? "🎤" : ""}
            status={microphonePermission}
            onclick={resetPermission}
            showName={showNames}
        />
        <PermissionButton
            name="Notification"
            icon={showIcons ? "🔔" : ""}
            status={notificationPermission}
            onclick={() => {
                debug(`Activate notification permission`);
                notifier.getPermissionToNotify(configStore.testNotify.value);
                trace(`Sent notification permission request`);
                notifier
                    .checkPermissionGranted()
                    .then(
                        (permission) => (notificationPermission = permission),
                    );
                trace(`Updated notification permission`);
            }}
            showName={showNames}
        />
    </div>
</div>
