<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Snippet } from "svelte";
    import Tab from "./ui/Tab.svelte";
    import WhisperOptions from "./WhisperOptions.svelte";
    import AppOptions from "./AppOptions.svelte";
    import DangerZone from "./DangerZone.svelte";

    interface MenuScreenProps {
        open?: boolean;
        title?: string;
        description?: string;
        class?: string;
    }

    let {
        open = $bindable(false),
        title = "Menu",
        description = "Menu for the application",
        class: className = "",
    }: MenuScreenProps = $props();
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger class="fixed top-0 btn btn-secondary btn-soft p-2 m-1"
        >Open Menu</Dialog.Trigger
    >
    <Dialog.Content class={`${className}`}>
        <div>
            <Dialog.Title><h2 class="">{title}</h2></Dialog.Title>
            <Dialog.Description>
                <span class="">{description}</span>
            </Dialog.Description>
        </div>
        <div class=" overflow-auto">
            <section class="tabs tabs-lift w-full">
                <Tab
                    value="tabs"
                    label="Configuration"
                    class="bg-base-100 border-base-300 p-6"
                    inputClass="not-checked:bg-base-200 not-checked:text-accent-content hover:font-bold hover:text-base-content border-base-300"
                    checked
                >
                    <div class="h-60 overflow-auto pr-6">
                        <WhisperOptions />
                    </div>
                </Tab>
                <Tab
                    value="tabs"
                    label="App Options"
                    class="bg-base-100 border-base-300 p-6"
                    inputClass="not-checked:bg-base-200 not-checked:text-accent-content hover:font-bold hover:text-base-content border-base-300"
                >
                    <div class="h-60 overflow-auto pr-6">
                        <AppOptions />
                    </div>
                </Tab>
                <Tab
                    value="tabs"
                    label="Danger Zone"
                    inputClass="input-ghost p-6 hover:bg-error hover:font-bold not-checked:bg-base-200 not-checked:text-warning-content checked:input-xl checked:text-warning"
                    class="bg-base-100 border-base-300 p-6"
                >
                    <div class="h-60 overflow-auto pr-6">
                        <DangerZone bind:isDialogOpen={open} />
                    </div>
                </Tab>
            </section>
        </div>
    </Dialog.Content>
</Dialog.Root>
