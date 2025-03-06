<!--
@component
Component based off of: <https://daisyui.com/components/theme-controller/>
-->
<script lang="ts">
    import { configStore } from "$lib/store";

    interface ThemeSwitcherProps {
        themes: {
            value: string;
            label?: string;
            isDefault?: boolean;
            kind?: "system" | "light" | "dark";
        }[];
        class?: string;
        listClass?: string;
        current?: "system" | "light" | "dark";
        direction?: "top" | "bottom" | "left" | "right";
    }

    let {
        themes,
        class: className,
        listClass,
        current = $bindable("system"),
        direction = "bottom",
    }: ThemeSwitcherProps = $props();

    async function setNewTheme(kind: "system" | "light" | "dark") {
        console.log(await configStore.getTheme());
        await configStore.setTheme(kind);
        console.log(await configStore.getTheme());
    }

    $effect(() => {
        let getTheme = async () => {
            const loadedTheme = await configStore.getTheme();
            console.log("THEME:", loadedTheme);
            if (loadedTheme) {
                current = loadedTheme;
                document
                    .getElementById("theme-list")
                    ?.childNodes.forEach((ele) => {
                        const inputNode = ele.firstChild;
                        if (!inputNode) return;
                        // @ts-ignore value does exist for input type for dropdown
                        const val = inputNode.value;
                        // NOTE: Assumes there is exactly one of each kind
                        // @ts-ignore check does exist for input type for dropdown
                        inputNode.checked = val === loadedTheme;
                    });
            }
        };

        getTheme();
    });
</script>

<div class={`dropdown dropdown-${direction} z-10 ${className}`}>
    <div tabindex="0" role="button" class="btn m-1">
        Theme:<span class="text-accent">{current}</span>
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
                    bind:group={current}
                    onclick={() => setNewTheme(theme.kind ?? "system")}
                />
            </li>
        {/each}
    </ul>
</div>
