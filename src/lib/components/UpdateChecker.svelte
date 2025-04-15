<script lang="ts">
    import {
        check,
        type DownloadEvent,
        type Update,
    } from "@tauri-apps/plugin-updater";
    import Button from "./ui/button/button.svelte";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { debug, error, info, trace, warn } from "@tauri-apps/plugin-log";
    import { configStore } from "$lib/store.svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";

    //                           min | s  | ms
    const UPDATE_CHECK_FREQUENCY = 5 * 60 * 1000;

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

    let updateIntervalId: number | undefined;
    let skipped = false;

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
            notifier.showToast("Could not apply update", "error");
            return;
        }

        // Once we start downloading, we don't have any updates
        hasUpdate = false;
        await autoUpdater.downloadAndInstall(progressCallback);
    }

    function afterCheck(updateInfo: Update | null) {
        if (updateInfo === null) {
            warn("There was no update given, ignoring update");
            return;
        }
        debug(`Update Info: ${JSON.stringify(updateInfo)}`);
        autoUpdater = updateInfo;
        if (!skipped) {
            if (configStore.autoApplyUpdates.value) {
                debug("Applying update immediately");
                applyUpdate();
            } else if (configStore.notifyOfUpdates.value) {
                notifier.confirmAction(
                    `Current version is ${updateInfo.currentVersion}, new version is ${updateInfo.version}`,
                    applyUpdate,
                    () => {
                        info("Skip new update");
                        skipped = true;
                    },
                    "New Update Available",
                    "Install",
                    "Skip",
                );
            }
        }
    }

    function onCheckError(err: any) {
        warn(`No update found for current target: ${err}`);
    }

    function createAutoChecker() {
        debug("Creating auto update checker");
        // Clear already running if there is one,
        // this prevents duplicates
        if (updateIntervalId !== undefined) {
            debug("Remove old auto checker");
            clearInterval(updateIntervalId);
        }
        updateIntervalId = setInterval(async () => {
            debug("Check again for update");
            check().then(afterCheck, onCheckError);
        }, UPDATE_CHECK_FREQUENCY);
    }

    $effect(() => {
        let unListenAutoCheckUpdate: UnlistenFn | undefined;
        const runCheck = async () => {
            await configStore.waitForStoreLoaded();
            await check().then(afterCheck, onCheckError);
            unListenAutoCheckUpdate = await listen("autoCheckUpdate", (e) => {
                const checked = e.payload as boolean;
                if (checked) {
                    createAutoChecker();
                } else {
                    debug("Removing update checker interval");
                    clearInterval(updateIntervalId);
                }
            });
            if (configStore.autoCheckForUpdates.value) {
                createAutoChecker();
            }
        };
        runCheck();
        return () => {
            clearInterval(updateIntervalId);
            unListenAutoCheckUpdate?.();
        };
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
            ? `: ${((currentUpdateCount / totalUpdateCount) * 100).toFixed(2)}%`
            : "..."}
    </div>
{/if}
