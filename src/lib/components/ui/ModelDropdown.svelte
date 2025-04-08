<!--
@component
Component to select the model to use
-->
<script lang="ts">
    import { commands } from "$lib/bindings";
    import {
        WHISPER_GGML_MODELS,
        DEFAULT_MODEL,
        MODELS_DIR,
    } from "$lib/constants";
    import { checkDownloadedModels, nameOfModel } from "$lib/myUtils";
    import { configStore } from "$lib/store.svelte";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { appLocalDataDir } from "@tauri-apps/api/path";
    import { debug, error } from "@tauri-apps/plugin-log";
    import { untrack } from "svelte";
    import Loading from "./Loading.svelte";

    interface ModelDropdownProps {
        class?: string;
        listClass?: string;
        direction?: "top" | "bottom" | "left" | "right";
    }

    let {
        class: className = "",
        listClass = "",
        direction = "bottom",
    }: ModelDropdownProps = $props();

    let baseModelDir = $state("");
    let isUpdating = $state(false);

    const selectableModels = $derived(
        WHISPER_GGML_MODELS.concat().filter(
            (model) =>
                configStore.downloadedModels.value.find(
                    (mPath) => mPath === model.relativePath,
                ) !== undefined,
        ),
    );

    $effect(() => {
        checkDownloadedModels().then((models) => {
            configStore.downloadedModels.value = models
                .filter((modelInfo) => modelInfo.downloaded)
                .map((modelInfo) => modelInfo.model.relativePath);
        });
        untrack(() => {
            appLocalDataDir().then((path) => (baseModelDir = path));
        });
    });

    async function removeModel() {
        debug("Will remove custom model");
        try {
            const result = await commands.updateModel(null, null);
            if (result.status === "error") {
                throw result.error;
            }
            notifier.showToast(
                "Model switched successfully",
                "",
                "success",
                "",
                5_000,
            );
        } catch (err) {
            error(`Could not unload custom model: ${err}`);
            notifier.showToast("Custom model could not removed", "", "warn");
        } finally {
            isUpdating = false;
        }
    }

    async function replaceModel(path: string, useGPU: boolean | null) {
        debug("Switching custom model");
        try {
            const result = await commands.updateModel(path, useGPU);
            if (result.status === "error") {
                throw result.error;
            }
            notifier.showToast(
                "Model switched successfully",
                "",
                "success",
                "",
                5_000,
            );
        } catch (err) {
            error(`Could not switch custom model: ${err}`);
            notifier.showToast("Could not switch custom model", "", "error");
        } finally {
            isUpdating = false;
        }
    }

    $effect(() => {
        if (baseModelDir) {
            untrack(() => {
                isUpdating = true;
            });
            if (configStore.currentModel.value === "default") {
                removeModel();
            } else {
                replaceModel(
                    `${baseModelDir}/${MODELS_DIR}/${configStore.currentModel.value}`,
                    configStore.useGPU.value,
                );
            }
        }
    });
</script>

<div class={`dropdown dropdown-${direction} ${className}`}>
    <div
        tabindex="0"
        role="button"
        class="btn m-1 btn-soft bg-info/10 hover:bg-info/70 w-full text-sm"
    >
        {#if isUpdating}
            <Loading color="accent" variant="spinner" />
        {/if}
        Model:
        <span class="text-accent"
            >{configStore.currentModel.value === "default"
                ? "Default"
                : nameOfModel(
                      selectableModels.find(
                          (model) =>
                              model.relativePath ===
                              configStore.currentModel.value,
                      )!,
                  ) || "Unknown Model"} &#9660;
        </span>
    </div>
    <ul
        id="model-list"
        class={`dropdown-content menu bg-base-300 rounded-box shadow-2xl w-full ${listClass}`}
    >
        <li>
            <input
                tabindex="0"
                type="radio"
                name="model-dropdown"
                class="btn btn-sm btn-block btn-ghost hover:btn-soft justify-start checked:border-primary checked:border-2 text-center"
                aria-label={`Default [${nameOfModel(DEFAULT_MODEL)}]`}
                value="default"
                disabled={isUpdating}
                checked={DEFAULT_MODEL.relativePath ===
                    configStore.currentModel.value}
                bind:group={configStore.currentModel.value}
            />
        </li>
        {#each selectableModels as model}
            <li>
                <input
                    tabindex="0"
                    type="radio"
                    name="model-dropdown"
                    class="btn btn-sm btn-block btn-ghost hover:btn-soft justify-start checked:border-primary checked:border-2 text-center"
                    aria-label={nameOfModel(model)}
                    value={model.relativePath}
                    disabled={isUpdating}
                    checked={model.relativePath ===
                        configStore.currentModel.value}
                    bind:group={configStore.currentModel.value}
                />
            </li>
        {/each}
    </ul>
</div>
