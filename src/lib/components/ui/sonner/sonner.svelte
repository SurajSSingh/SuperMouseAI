<!--
@component
- Wrapper for svelte-sonner component
- See docs at <https://next.shadcn-svelte.com/docs/components/sonner>
-->

<script lang="ts">
    import {
        Toaster as Sonner,
        type ToasterProps as SonnerProps,
    } from "svelte-sonner";

    let restProps: SonnerProps = $props();
</script>

<Sonner
    class="toaster group"
    toastOptions={{
        classes: {
            toast: "group group-[.toaster]:bg-background group-[.toaster]:text-foreground group-[.toaster]:border-border group-[.toaster]:shadow-lg",
            description: "group-[.toast]:text-muted-foreground",
            actionButton:
                "group-[.toast]:bg-primary group-[.toast]:text-primary-foreground",
            cancelButton:
                "group-[.toast]:bg-muted group-[.toast]:text-muted-foreground",
        },
    }}
    {...restProps}
/>
