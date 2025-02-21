<script lang="ts">
    interface ThemeSwitcherProps {
        themes: {
            value: string;
            label?: string;
            isDefault?: boolean;
            kind?: "system" | "light" | "dark";
        }[];
        class?: string;
        current?: "system" | "light" | "dark";
    }

    let {
        themes,
        class: className,
        current = $bindable("system"),
    }: ThemeSwitcherProps = $props();
</script>

<div class={`dropdown ${className}`}>
    <div tabindex="0" role="button" class="btn m-1">
        Theme
        <svg
            width="12px"
            height="12px"
            class="inline-block h-2 w-2 fill-current opacity-60"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 2048 2048"
        >
            <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"
            ></path>
        </svg>
    </div>
    <ul
        class="dropdown-content bg-base-300 rounded-box z-1 w-52 p-2 shadow-2xl"
    >
        {#each themes as theme}
            <li>
                <input
                    type="radio"
                    name="theme-dropdown"
                    class="theme-controller w-full btn btn-sm btn-block btn-ghost hover:btn-soft justify-start"
                    aria-label={theme.label || theme.value}
                    value={theme.value}
                    checked={theme?.isDefault ?? false}
                    bind:group={current}
                />
            </li>
        {/each}
    </ul>
</div>
