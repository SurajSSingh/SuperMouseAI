<script lang="ts">
    import {
        check,
        type DownloadEvent,
        type Update,
    } from "@tauri-apps/plugin-updater";
    import Button from "./ui/button/button.svelte";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { debug, error, info, warn } from "@tauri-apps/plugin-log";

    interface Props {
        hasUpdate?: boolean;
        class?: string;
    }

    let { hasUpdate = $bindable(false), class: className = "" }: Props =
        $props();

    let autoUpdater: Update | undefined = $state();
    let isUpdating = $state(false);
    let totalUpdateCount = $state(0);
    let currentUpdateCount = $state(0);

    function progressCallback(event: DownloadEvent) {
        switch (event.event) {
            case "Started":
                info("Started applying app update");
                debug(`Content Length of update: ${event.data.contentLength}`);
                isUpdating = true;
                totalUpdateCount = event.data.contentLength ?? 0;
                break;
            case "Progress":
                debug(
                    `Update recieved chunk of length: ${event.data.chunkLength}`,
                );
                currentUpdateCount += event.data.chunkLength;
                break;
            case "Finished":
                info("Finished applying app update");
                isUpdating = false;
                break;
            default:
                warn("Reached unreachable case in showProgress");
                isUpdating = false;
                // TODO(@): May want to do a cursory check again
                hasUpdate = false;
                break;
        }
    }

    async function applyUpdate() {
        if (!autoUpdater) {
            error(`Update class not defined to run auto-update.`);
            hasUpdate = false;
            notifier.showError("Could not apply update");
            return;
        }

        // Once we start downloading, we don't have any updates
        hasUpdate = false;
        await autoUpdater.downloadAndInstall(progressCallback);
    }

    function afterCheck(updateInfo: Update | null) {
        if (updateInfo) {
            autoUpdater = updateInfo;
            hasUpdate = updateInfo.available;
            if (hasUpdate) {
                notifier.confirmAction(
                    `Current version is ${updateInfo.currentVersion}, new version is ${updateInfo.version}`,
                    applyUpdate,
                    () => {},
                    "New Update Available",
                    "Install",
                    "Skip",
                );
            }
        }
    }

    $effect(() => {
        check().then(afterCheck, (err) => {
            warn(`No update found for current target: ${err}`);
        });
    });
</script>

{#if hasUpdate}
    <Button
        onclick={applyUpdate}
        class={className}
        variant="ghost"
        color="warning">Click here to update to {autoUpdater?.version}!</Button
    >
{:else if isUpdating}
    <div class={`text-center ${className}`}>
        <i class=" loading loading-bars"></i>
        Downloading Update{totalUpdateCount > 0
            ? `: ${((currentUpdateCount / totalUpdateCount) * 100).toFixed(2)}% (${currentUpdateCount}/${totalUpdateCount})`
            : "..."}
    </div>
{/if}
