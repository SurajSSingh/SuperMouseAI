<script lang="ts">
    import { Label } from "bits-ui";
    import Switch from "./switch/switch.svelte";
    import type { Snippet } from "svelte";
    import type { ButtonColor, ButtonSize } from "./button";
    import type { ChangeEventHandler } from "svelte/elements";

    interface ToggleSwitchProps {
        label?: string | Snippet;
        color?: ButtonColor;
        size?: ButtonSize;
        checked?: boolean;
        indeterminate?: boolean;
        checkedClass?: string;
        uncheckedClass?: string;
        onchange?: ChangeEventHandler<HTMLInputElement>;
        disabled?: boolean;
    }

    let {
        label,
        color = "default",
        size = "md",
        checked = $bindable(false),
        indeterminate = false,
        checkedClass = "",
        uncheckedClass = "",
        disabled = false,
        onchange,
    }: ToggleSwitchProps = $props();
</script>

<div>
    {#if typeof label === "string"}
        <Label.Root>{label}</Label.Root>
    {:else}
        {@render label?.()}
    {/if}
    <input
        type="checkbox"
        name="x"
        id="x"
        class={`toggle ${color !== "default" ? (color !== "destructive" ? `toggle-${color}` : "toggle-error") : ""} toggle-${size} ${uncheckedClass} ${checkedClass
            .split(" ")
            .map((cls) => (cls.startsWith("checked:") ? cls : `checked:${cls}`))
            .join(" ")}`}
        bind:checked
        {indeterminate}
        {onchange}
        {disabled}
    />
</div>
