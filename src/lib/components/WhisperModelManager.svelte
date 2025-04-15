<script lang="ts">
    import {
        BASE_LOCAL_APP_DIR,
        MODEL_BASE_URL,
        MODELS_DIR,
    } from "$lib/constants";
    import type { WhisperModelInfo } from "$lib/types";
    import {
        create as createFile,
        exists,
        mkdir,
        remove as removeFile,
    } from "@tauri-apps/plugin-fs";
    import { debug, error, info, warn } from "@tauri-apps/plugin-log";
    import Button from "./ui/button/button.svelte";
    import Loading from "./ui/Loading.svelte";
    import {
        checkDownloadedModels,
        convertBPSToHuman,
        convertBytesToHuman,
        nameOfModel,
        sizeOfModel,
    } from "$lib/myUtils";
    import { configStore } from "$lib/store.svelte";
    import { download } from "@tauri-apps/plugin-upload";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { appLocalDataDir } from "@tauri-apps/api/path";
    import ToggleSwitch from "./ui/ToggleSwitch.svelte";

    // HACK: This is because it is not exported from types
    interface ProgressPayload {
        progress: number;
        progressTotal: number;
        total: number;
        transferSpeed: number;
    }

    let downloadedModels: WhisperModelInfo[] = $state([]);
    let notDownloadedModels: WhisperModelInfo[] = $state([]);
    let value = $state("default");
    let isProcessingModel = $state(false);
    let downloadProgress: ProgressPayload | null = $state(null);

    const isDefault = $derived(value === "default");
    const isDownloaded = $derived(
        configStore.downloadedModels.value.find((model) => model === value) !==
            undefined,
    );
    const filteredNotDownloadedModels = $derived(
        notDownloadedModels.filter(includeModelBasedOnFilter),
    );

    $effect(() => {
        const onMount = async () => {
            info("Checking for models");
            debug("Making sure models directory exists");
            const modelsExists = await exists(MODELS_DIR, BASE_LOCAL_APP_DIR);
            if (!modelsExists) {
                debug("Making models directory");
                await mkdir(MODELS_DIR, BASE_LOCAL_APP_DIR);
            }
            debug("Check for valid models");
            const modelCheckPromises = await checkDownloadedModels();
            downloadedModels = modelCheckPromises
                .filter((modelInfo) => modelInfo.downloaded)
                .map((modelInfo) => modelInfo.model);
            configStore.downloadedModels.value = downloadedModels.map(
                (model) => model.relativePath,
            );
            notDownloadedModels = modelCheckPromises
                .filter((modelInfo) => !modelInfo.downloaded)
                .map((modelInfo) => modelInfo.model);
        };
        onMount();
        return () => {};
    });

    /**
     * Given the filters in `configStore`, check if a model is to be included based on its properties
     * @param model: The model to check
     * @returns Whether the model should be included
     */
    export function includeModelBasedOnFilter(
        model: WhisperModelInfo,
    ): boolean {
        if (
            model.isEnglishOnly === true &&
            !configStore.includeEnglishOnlyModels.value
        ) {
            return false;
        }
        if (
            model.isSuperceded === true &&
            !configStore.includeObsoleteModels.value
        ) {
            return false;
        }
        if (
            model.quantizeType !== "full" &&
            !configStore.includeQuantizedModels.value
        ) {
            return false;
        }
        return true;
    }

    async function downloadModel() {
        info("Start Downloading new model");
        isProcessingModel = true;
        debug("Get model to download");
        const addedModel = notDownloadedModels.find(
            (model) => model.relativePath === value,
        );
        if (addedModel === undefined) {
            error(
                "Model cannot be downloaded because it is not found in downloadable models.",
            );
            return;
        }
        debug(`Creating file at: ${MODELS_DIR}/${addedModel.relativePath}`);
        await createFile(
            `${MODELS_DIR}/${addedModel.relativePath}`,
            BASE_LOCAL_APP_DIR,
        );
        debug(`Downloading Model: ${addedModel.relativePath}`);
        try {
            await download(
                `${MODEL_BASE_URL}/${addedModel.relativePath}`,
                `${await appLocalDataDir()}/${MODELS_DIR}/${addedModel.relativePath}`,
                (progress) => {
                    downloadProgress = progress;
                },
            );
            // NOTE: Because JS does not have `else`, this must
            //       be placed inside the try block, even though
            //       below code should not throw error.
            downloadedModels.push(addedModel);
            configStore.addModel(addedModel);
            notDownloadedModels = notDownloadedModels.filter(
                (model) => model.relativePath !== value,
            );
            info(
                `New model ${nameOfModel(addedModel)} (${addedModel.relativePath}) ready to use`,
            );
            notifier.showToast(
                "Model has been downloaded, and is ready to use",
                "success",
                { duration: 5_000 },
            );
        } catch (err: any) {
            removeFile(
                `${MODELS_DIR}/${addedModel.relativePath}`,
                BASE_LOCAL_APP_DIR,
            );
            error(err.toString());
            notifier.showToast("Could not download the model", "error", {
                sound: "alert",
                duration: 10_000,
            });
        } finally {
            isProcessingModel = false;
            downloadProgress = null;
        }
    }

    async function removeModel() {
        info("Removing a model");
        downloadProgress = null;
        isProcessingModel = true;
        const removingModel = downloadedModels.find(
            (model) => model.relativePath === value,
        );
        if (removingModel === undefined) {
            warn(
                "Model cannot be removed because it is not found in models folder.",
            );
            return;
        }
        debug(`Removing Model: ${removingModel.relativePath}`);
        try {
            await removeFile(
                `${MODELS_DIR}/${removingModel.relativePath}`,
                BASE_LOCAL_APP_DIR,
            );
        } catch (err) {
            error(`Could not remove model: ${err}`);
            notifier.showToast("Could not remove the given model", "error", {
                sound: "alert",
                duration: 10_000,
            });
            return;
        }
        notDownloadedModels.push(removingModel);
        downloadedModels = downloadedModels.filter(
            (model) => model.relativePath !== value,
        );
        configStore.removeModel(removingModel);
        isProcessingModel = false;
        info(
            `Model ${nameOfModel(removingModel)} (${removingModel.relativePath}) removed successfully`,
        );
        notifier.showToast("Model has been removed", "success", {
            duration: 5_000,
        });
    }
</script>

<fieldset class="fieldset">
    <legend class="fieldset-legend">Model Selection</legend>
    <details class="my-2">
        <summary class="font-semibold">More Model Options</summary>
        <p class="my-2 text-sm">Add more models for downloading</p>
        <div class="mb-2">
            <ToggleSwitch
                label="Include English-only models"
                bind:checked={configStore.includeEnglishOnlyModels.value}
            />
            <p class="fieldset-label">
                Show models which are more accurate at transcribing for English.
            </p>
        </div>
        <div class="mb-2">
            <ToggleSwitch
                label="Include Quantized (Compressed) models"
                bind:checked={configStore.includeQuantizedModels.value}
            />
            <p class="fieldset-label">
                Show models which are smaller at the cost of overall accuracy.
            </p>
        </div>
        <div class="mb-2">
            <ToggleSwitch
                label="Include Old models"
                bind:checked={configStore.includeObsoleteModels.value}
            />
            <p class="fieldset-label">
                Show models which have been replaced by newer models.
            </p>
        </div>
    </details>
    <select class="select select-sm" bind:value disabled={isProcessingModel}>
        <option value="default">Select a model to manage</option>
        <optgroup label="Downloaded Models">
            <option value="" disabled
                >Default (Tiny, High Compression, English only) (32.2 MB)</option
            >
            {#each downloadedModels as model}
                <option value={model.relativePath}
                    >{nameOfModel(model)} ({sizeOfModel(model)})</option
                >
            {/each}
        </optgroup>
        <optgroup label="Able to Download">
            {#each filteredNotDownloadedModels as model}
                <option value={model.relativePath}
                    >{nameOfModel(model)} ({sizeOfModel(model)})</option
                >
            {/each}
        </optgroup>
    </select>
    <span class="fieldset-label"
        >Model ordered from smallest to largest, choose the biggest you can for
        your machine.
    </span>
    {#if isDownloaded}
        <Button
            onclick={removeModel}
            color="destructive"
            variant="soft"
            disabled={isProcessingModel}
        >
            {#if isProcessingModel}
                <Loading color="warning" variant="spinner" />
            {/if}
            Remove Model</Button
        >
    {:else if !isDefault}
        <Button
            onclick={downloadModel}
            color="info"
            variant="soft"
            disabled={isProcessingModel}
        >
            {#if isProcessingModel}
                <Loading color="info" variant="spinner" />
            {/if}
            Download
        </Button>
        {#if isProcessingModel && downloadProgress}
            <progress
                class={`progress w-full ${isProcessingModel ? "progress-primary" : "progress-error"}`}
                aria-label="model-download-progress"
                value={downloadProgress.progressTotal}
                max={downloadProgress.total}
                >{downloadProgress.progressTotal}</progress
            >
            <div class="text-center">
                <span>
                    {convertBytesToHuman(
                        downloadProgress.progressTotal,
                    )}/{convertBytesToHuman(downloadProgress.total)}
                </span>
                <span>
                    (Speed: {convertBPSToHuman(
                        downloadProgress.transferSpeed /
                            1000 /* HACK: Not sure why it is multiplied by 1000, but it now give corrected speed */,
                    )})
                </span>
            </div>
        {/if}
    {/if}
</fieldset>
