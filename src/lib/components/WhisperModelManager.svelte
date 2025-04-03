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
    import { checkDownloadedModels } from "$lib/myUtils";
    import { configStore } from "$lib/store.svelte";
    import { download } from "@tauri-apps/plugin-upload";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { appLocalDataDir } from "@tauri-apps/api/path";

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
            downloadedModels.push(addedModel);
            configStore.addModel(addedModel);
            notDownloadedModels = notDownloadedModels.filter(
                (model) => model.relativePath !== value,
            );
            info(
                `New model ${addedModel.name} (${addedModel.relativePath}) ready to use`,
            );
            notifier.showToast(
                "Model has been downloaded, and is ready to use",
                "",
                "success",
                "",
                5_000,
            );
        } catch (err: any) {
            removeFile(
                `${MODELS_DIR}/${addedModel.relativePath}`,
                BASE_LOCAL_APP_DIR,
            );
            error(err.toString());
            notifier.showToast(
                "Could not download the model",
                "",
                "error",
                "alert",
                10_000,
            );
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
        await removeFile(
            `${MODELS_DIR}/${removingModel.relativePath}`,
            BASE_LOCAL_APP_DIR,
        );
        notDownloadedModels.push(removingModel);
        downloadedModels = downloadedModels.filter(
            (model) => model.relativePath !== value,
        );
        configStore.removeModel(removingModel);
        isProcessingModel = false;
        info(
            `Model ${removingModel.name} (${removingModel.relativePath}) removed successfully`,
        );
    }
</script>

<fieldset class="fieldset">
    <legend class="fieldset-legend">Browsers</legend>
    <select class="select select-sm" bind:value disabled={isProcessingModel}>
        <option value="default">Select a model to manage</option>
        <optgroup label="Downloaded Models">
            <option value="" disabled
                >Default (Tiny, High Compression, English only) (32.2 MB)</option
            >
            {#each downloadedModels as model}
                <option value={model.relativePath}
                    >{model.name} ({model.approxSize})</option
                >
            {/each}
        </optgroup>
        <optgroup label="Able to Download">
            {#each notDownloadedModels as model}
                <option value={model.relativePath}
                    >{model.name} ({model.approxSize})</option
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
            color="neutral"
            variant="soft"
            disabled={isProcessingModel}
        >
            {#if isProcessingModel}
                <Loading color="info" variant="spinner" />
            {/if}
            Download
        </Button>
        {#if downloadProgress}
            <progress
                class={`progress w-full ${isProcessingModel ? "progress-primary" : "progress-error"}`}
                value={downloadProgress.progressTotal}
                max={downloadProgress.total}
            ></progress>
            <span
                >{downloadProgress.progress} ({downloadProgress.transferSpeed})</span
            >
        {/if}
    {/if}
</fieldset>
