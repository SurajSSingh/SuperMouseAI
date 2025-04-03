<script lang="ts">
    import {
        BASE_LOCAL_APP_DIR,
        MODEL_BASE_URL,
        MODELS_DIR,
    } from "$lib/constants";
    import type { WhisperModelInfo } from "$lib/types";
    import { exists, mkdir, remove, writeFile } from "@tauri-apps/plugin-fs";
    import { debug, error, info } from "@tauri-apps/plugin-log";
    import Button from "./ui/button/button.svelte";
    import Loading from "./ui/Loading.svelte";
    import { fetch } from "@tauri-apps/plugin-http";
    import { blobToBytes, checkDownloadedModels } from "$lib/myUtils";
    import { configStore } from "$lib/store.svelte";

    let downloadedModels: WhisperModelInfo[] = $state([]);
    let notDownloadedModels: WhisperModelInfo[] = $state([]);
    let value = $state("default");
    let isProcessingModel = $state(false);

    const isDefault = $derived(value === "default");
    const isDownloaded = $derived(
        configStore.downloadedModels.value.find((model) => model === value) !==
            undefined,
    );

    $effect(() => {
        const onMount = async () => {
            info("Checking for models");
            debug("Make sure models directory exists");
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
        debug(`Downloading Model: ${addedModel.relativePath}`);
        const response = await fetch(
            `${MODEL_BASE_URL}/${addedModel.relativePath}`,
        );
        console.dir(response);
        const file = await response.blob();
        await writeFile(
            `${MODELS_DIR}/${addedModel.relativePath}`,
            await blobToBytes(file),
            BASE_LOCAL_APP_DIR,
        );
        downloadedModels.push(addedModel);
        configStore.addModel(addedModel);
        notDownloadedModels = notDownloadedModels.filter(
            (model) => model.relativePath !== value,
        );
        isProcessingModel = false;
        info(
            `New model ${addedModel.name} (${addedModel.relativePath}) ready to use`,
        );
    }

    async function removeModel() {
        info("Removing a model");
        isProcessingModel = true;
        const removingModel = downloadedModels.find(
            (model) => model.relativePath === value,
        );
        if (removingModel === undefined) {
            error(
                "Model cannot be downloaded because it is not found in downloadable models.",
            );
            return;
        }
        debug(`Removing Model: ${removingModel.relativePath}`);
        await remove(
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
    {/if}
</fieldset>
