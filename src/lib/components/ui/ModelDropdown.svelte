<!--
@component
Component to select the model to use
-->
<script lang="ts">
    import { WHISPER_GGML_MODELS, DEFAULT_MODEL } from "$lib/constants";
    import { checkDownloadedModels } from "$lib/myUtils";
    import { configStore } from "$lib/store.svelte";

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
    });
</script>

<div class={`dropdown dropdown-${direction} z-5 ${className}`}>
    <div
        tabindex="0"
        role="button"
        class="btn m-1 btn-soft bg-info/10 hover:bg-info/70 w-full text-sm"
    >
        Model: <span class="text-accent"
            >{configStore.currentModel.value === "default"
                ? "Default"
                : selectableModels.find(
                      (model) =>
                          model.relativePath === configStore.currentModel.value,
                  )?.name || "Unknown Model"} &#9660;
        </span>
    </div>
    <ul
        id="model-list"
        class={`dropdown-content menu bg-base-300 rounded-box z-1 shadow-2xl w-full ${listClass}`}
    >
        <li>
            <input
                tabindex="0"
                type="radio"
                name="model-dropdown"
                class="btn btn-sm btn-block btn-ghost hover:btn-soft justify-start checked:border-primary checked:border-2 text-center"
                aria-label={`Default [${DEFAULT_MODEL.name}]`}
                value="default"
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
                    aria-label={model.name}
                    value={model.relativePath}
                    checked={model.relativePath ===
                        configStore.currentModel.value}
                    bind:group={configStore.currentModel.value}
                />
            </li>
        {/each}
    </ul>
</div>
