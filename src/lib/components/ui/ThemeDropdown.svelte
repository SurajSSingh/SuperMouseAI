<!--
@component
Component based off of: <https://daisyui.com/components/theme-controller/>
-->
<script lang="ts">
    import { configStore } from "$lib/store.svelte";

    const THEMES = [
        {
            value: "system",
            label: "System",
            isDefault: true,
            kind: "system" as const,
        },
        { value: "light", label: "Light", kind: "light" as const },
        { value: "dark", label: "Dark", kind: "dark" as const },
    ];

    interface ThemeSwitcherProps {
        class?: string;
        listClass?: string;
        direction?: "top" | "bottom" | "left" | "right";
    }

    let {
        class: className,
        listClass,
        direction = "bottom",
    }: ThemeSwitcherProps = $props();
</script>

<div class={`dropdown dropdown-${direction} z-10 ${className}`}>
    <div tabindex="0" role="button" class="btn m-1">
        Theme:<span class="text-accent">{configStore.theme.value} &#9660;</span>
    </div>
    <ul
        id="theme-list"
        class={`dropdown-content bg-base-300 rounded-box z-1 shadow-2xl ${listClass}`}
    >
        {#each THEMES as theme}
            <li>
                <input
                    tabindex="0"
                    type="radio"
                    name="theme-dropdown"
                    class="theme-controller btn btn-sm btn-block btn-ghost hover:btn-soft justify-start"
                    aria-label={theme.label || theme.value}
                    value={theme.value}
                    checked={theme?.isDefault ?? false}
                    bind:group={configStore.theme.value}
                />
            </li>
        {/each}
    </ul>
</div>
