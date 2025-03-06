<!--
@component
Component based off of: <https://daisyui.com/components/theme-controller/>
-->
<script lang="ts">
    import { configStore } from "$lib/store.svelte";

    interface ThemeSwitcherProps {
        themes: {
            value: string;
            label?: string;
            isDefault?: boolean;
            kind?: "system" | "light" | "dark";
        }[];
        class?: string;
        listClass?: string;
        direction?: "top" | "bottom" | "left" | "right";
    }

    let {
        themes,
        class: className,
        listClass,
        direction = "bottom",
    }: ThemeSwitcherProps = $props();
</script>

<div class={`dropdown dropdown-${direction} z-10 ${className}`}>
    <div tabindex="0" role="button" class="btn m-1">
        Theme:<span class="text-accent">{configStore.theme}</span>
    </div>
    <ul
        id="theme-list"
        class={`dropdown-content bg-base-300 rounded-box z-1 shadow-2xl ${listClass}`}
    >
        {#each themes as theme}
            <li>
                <input
                    tabindex="0"
                    type="radio"
                    name="theme-dropdown"
                    class="theme-controller btn btn-sm btn-block btn-ghost hover:btn-soft justify-start"
                    aria-label={theme.label || theme.value}
                    value={theme.value}
                    checked={theme?.isDefault ?? false}
                    bind:group={configStore.theme}
                />
            </li>
        {/each}
    </ul>
</div>
