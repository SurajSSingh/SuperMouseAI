<script lang="ts">
    import { Collapsible } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props {
        titleTag?: "h2" | "h3" | "h4" | "h5" | "h6" | "p";
        title: string;
        subtitle?: string;
        children?: Snippet;
        class?: string;
        titleClass?: string;
        legendClass?: string;
        wrapperClass?: string;
        open?: boolean;
    }

    let {
        open = $bindable(true),
        title,
        titleTag = "p",
        subtitle,
        class: className = "bg-base-200 border border-base-300 rounded-box",
        titleClass = "font-semibold mb-0 text-center text-xs",
        legendClass = "px-4",
        wrapperClass,
        children,
    }: Props = $props();
</script>

<fieldset class={`fieldset collapse collapse-plus ${className}`}>
    <legend class={`fieldset-legend ${legendClass}`}>{title}</legend>
    <input type="checkbox" bind:checked={open} />
    <svelte:element this={titleTag} class={`collapse-title ${titleClass}`}>
        {subtitle ? `${subtitle} | ` : ""}Click to {open ? "close" : "open"}
    </svelte:element>
    <div class={`collapse-content mt-0 ${wrapperClass}`}>
        {@render children?.()}
    </div>
</fieldset>
