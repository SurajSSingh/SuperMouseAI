<script lang="ts">
    import { notifier } from "$lib/notificationSystem.svelte";
    import PermissionButton from "$lib/components/PermissionButton.svelte";
    import { configStore } from "$lib/store.svelte";
    import { debug, trace } from "@tauri-apps/plugin-log";
    import MicrophoneDropdown from "./ui/MicrophoneDropdown.svelte";

    interface PermissionsBarProps {
        showNames?: boolean;
        showIcons?: boolean;
    }

    let { showNames = true, showIcons = true }: PermissionsBarProps = $props();

    $effect(() => {
        debug(`Update permissions for Permission Bar component`);
        trace(`Permissions allowed
        - Notifications = ${configStore.useSystemNotification.value}
        - Sound = ${configStore.enabledSound.value}
        `);
    });
</script>

<div>
    <div
        class="grid grid-cols-1 gap-1 lg:gap-2 xl:gap-3 sm:grid-cols-2 justify-items-center place-items-center items-stretch"
    >
        <MicrophoneDropdown
            name="Microphone"
            icon={showIcons ? "🎤" : ""}
            showName={showNames}
            class="col-span-full"
        />
        <PermissionButton
            name="Sound"
            icon={showIcons
                ? configStore.enabledSound.value
                    ? "🔊"
                    : "🔇"
                : ""}
            status={configStore.enabledSound.value}
            onclick={() => {
                notifier.setEnabledSound("toggle");
                notifier.playSound();
            }}
            showName={showNames}
        />
        <PermissionButton
            name="Notification"
            icon={showIcons
                ? configStore.useSystemNotification.value
                    ? "🔔"
                    : "🔕"
                : ""}
            status={configStore.useSystemNotification.value}
            onclick={() => {
                if (configStore.useSystemNotification.value) {
                    debug("Turning off system notifications");
                    configStore.useSystemNotification.value = false;
                } else {
                    debug(`Activate notification permission`);
                    notifier.getPermissionToNotify(
                        configStore.testNotify.value,
                    );
                    trace(`Sent notification permission request`);
                    notifier
                        .checkPermissionGranted()
                        .then(
                            (permission) =>
                                (configStore.useSystemNotification.value =
                                    permission),
                        );
                }
                trace(`Updated notification permission`);
            }}
            showName={showNames}
        />
    </div>
</div>
