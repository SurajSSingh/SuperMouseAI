<script lang="ts">
    import { WHISPER_GGML_MODELS } from "$lib/constants";
    import type { WhisperModelInfo } from "$lib/types";
    import { BaseDirectory, exists, mkdir } from "@tauri-apps/plugin-fs";
    import { debug, error, info } from "@tauri-apps/plugin-log";
    import Button from "./ui/button/button.svelte";
    import Loading from "./ui/Loading.svelte";

    const MODELS_DIR = "models";

    let downloadedModels: WhisperModelInfo[] = $state([]);
    let notDownloadedModels: WhisperModelInfo[] = $state([]);
    let value = $state("default");
    let isProcessingModel = $state(false);

    const isDefault = $derived(value === "default");
    const isDownloaded = $derived(
        downloadedModels.find((model) => model.relativePath === value) !==
            undefined,
    );

    $effect(() => {
        const onMount = async () => {
            info("Checking for models");
            debug("Make sure models directory exists");
            const modelsExists = await exists(MODELS_DIR, {
                baseDir: BaseDirectory.AppLocalData,
            });
            if (!modelsExists) {
                debug("Making models directory");
                await mkdir(MODELS_DIR, {
                    baseDir: BaseDirectory.AppLocalData,
                });
            }
            debug("Check for valid models");
            const modelCheckPromises = await Promise.all(
                WHISPER_GGML_MODELS.map(async (model) => {
                    return {
                        model,
                        downloaded: await exists(
                            `${MODELS_DIR}/${model.relativePath}`,
                            { baseDir: BaseDirectory.AppLocalData },
                        ),
                    };
                }),
            );
            downloadedModels = modelCheckPromises
                .filter((modelInfo) => modelInfo.downloaded)
                .map((modelInfo) => modelInfo.model);
            notDownloadedModels = modelCheckPromises
                .filter((modelInfo) => !modelInfo.downloaded)
                .map((modelInfo) => modelInfo.model);
        };
        onMount();
        return () => {};
    });

    async function downloadModel() {
        isProcessingModel = true;
        const addedModel = notDownloadedModels.find(
            (model) => model.relativePath === value,
        );
        if (addedModel === undefined) {
            error(
                "Model cannot be downloaded because it is not found in downloadable models.",
            );
            return;
        }
        await new Promise((resolve) => setTimeout(() => resolve(0), 3000));
        downloadedModels.push(addedModel);
        notDownloadedModels = notDownloadedModels.filter(
            (model) => model.relativePath !== value,
        );
        isProcessingModel = false;
    }

    async function removeModel() {
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
        await new Promise((resolve) => setTimeout(() => resolve(0), 3000));
        notDownloadedModels.push(removingModel);
        downloadedModels = downloadedModels.filter(
            (model) => model.relativePath !== value,
        );
        isProcessingModel = false;
    }
</script>

<fieldset class="fieldset">
    <legend class="fieldset-legend">Browsers</legend>
    <select class="select" bind:value disabled={isProcessingModel}>
        <option value="default">Default (77 Mb)</option>
        <optgroup label="Other Downloaded Models">
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
