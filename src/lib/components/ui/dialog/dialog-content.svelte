<script lang="ts">
    import {
        Dialog as DialogPrimitive,
        type WithoutChildrenOrChild,
    } from "bits-ui";
    import type { Snippet } from "svelte";
    import * as Dialog from "./index.js";
    import { cn } from "$lib/utils.js";

    let {
        ref = $bindable(null),
        class: className,
        portalProps,
        children,
        ...restProps
    }: WithoutChildrenOrChild<DialogPrimitive.ContentProps> & {
        portalProps?: DialogPrimitive.PortalProps;
        children: Snippet;
    } = $props();
</script>

<Dialog.Portal {...portalProps}>
    <Dialog.Overlay />
    <DialogPrimitive.Content
        bind:ref
        class={cn(
            "data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] fixed left-[50%] top-[50%] z-50 grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-4 border p-6 shadow-lg duration-200 sm:rounded-lg",
            "bg-base-200",
            className,
        )}
        {...restProps}
    >
        {@render children?.()}
        <DialogPrimitive.Close
            class="btn btn-error absolute top-0 right-0 mt-1 mr-1"
            >Close
        </DialogPrimitive.Close>
    </DialogPrimitive.Content>
</Dialog.Portal>
