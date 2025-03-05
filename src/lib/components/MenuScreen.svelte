<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import type { Snippet } from "svelte";

    interface MenuScreenProps {
        open?: boolean;
        title?: string;
        description?: string;
        class?: string;
        children?: Snippet;
    }

    let {
        open = $bindable(false),
        title = "Menu",
        description = "Menu for the application",
        class: className = "",
        children,
    }: MenuScreenProps = $props();
</script>

<Dialog.Root bind:open>
    <Dialog.Trigger class="fixed top-0 btn btn-secondary btn-soft p-2 m-1"
        >Open Menu</Dialog.Trigger
    >
    <Dialog.Content class={`${className}`}>
        <div>
            <Dialog.Title><h2>{title}</h2></Dialog.Title>
            <Dialog.Description>
                {description}
            </Dialog.Description>
        </div>
        <div class=" overflow-auto">
            {@render children?.()}
        </div>
    </Dialog.Content>
</Dialog.Root>
