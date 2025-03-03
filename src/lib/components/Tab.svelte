<!-- 
@component

See docs at <https://daisyui.com/components/tab/> 
-->
<script lang="ts">
    import type { Snippet } from "svelte";

    interface TabProps {
        value: string;
        checked?: boolean;
        label?: string;
        children: Snippet;
        class?: string;
        inputClass?: string;
    }

    let {
        value,
        checked,
        label,
        class: className,
        inputClass,
        children,
    }: TabProps = $props();
</script>

<input
    type="radio"
    name={value}
    class={`tab ${inputClass}`}
    aria-label={label || `Tab ${value}`}
    {checked}
/>
<div class={`tab-content ${className}`}>
    {@render children()}
</div>
