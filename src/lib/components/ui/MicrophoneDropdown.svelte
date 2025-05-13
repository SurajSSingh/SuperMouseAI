<!--
@component
Component to check microphone from backend
-->
<script lang="ts">
    import { commands } from "$lib/bindings";
    import { notifier } from "$lib/notificationSystem.svelte";
    import { configStore } from "$lib/store.svelte";
    import { debug, error, info, warn } from "@tauri-apps/plugin-log";
    import Status from "./Status.svelte";

    interface ThemeSwitcherProps {
        name: string;
        class?: string;
        listClass?: string;
        direction?: "top" | "bottom" | "left" | "right";
        useIcon?: boolean;
        showStatusAsEmoji?: boolean;
        showName?: boolean;
        status?: boolean | null;
        icon?: string;
    }

    let {
        name,
        class: className,
        listClass,
        direction = "bottom",
        useIcon = true,
        status,
        icon,
        showName = true,
        showStatusAsEmoji = false,
    }: ThemeSwitcherProps = $props();

    let inputDevices: string[] = $state([]);
    let selectedDevice = $state(-1);

    const statusColor = $derived(selectedDevice >= 0 ? "success" : "error");
    const statusValue = $derived(selectedDevice >= 0 ? "granted" : "denied");
    const statusIcon = $derived(selectedDevice >= 0 ? "✅" : "❌");

    async function getAudioInputs(): Promise<void> {
        const result = await commands.getInputDevices();
        if (result.status === "error") {
            error(`Error getting devices: ${result.error}`);
            notifier.showToast("Could not get input audio devices.", "error");
            return;
        }
        inputDevices = result.data;
        const default_mic_res = await commands.getCurrentInputDevice();
        if (default_mic_res.status === "error") {
            warn(
                `Error getting default input device: ${default_mic_res.error}`,
            );
            notifier.showToast("Could not get default microphone.", "warn");
            return;
        }
        if (default_mic_res.data) {
            debug("Find name in input devices and assign it.");
            selectedDevice = inputDevices.findIndex(
                (name) => name === default_mic_res.data,
            );
        }
    }

    async function onclick(): Promise<void> {
        if (selectedDevice < 0) return;
        const result = await commands.setInputDevice(selectedDevice);
        if (result.status === "error") {
            error(`Error setting devices: ${result.error}`);
            notifier.showToast("Could not set input audio device.", "error");
        } else if (!result.data) {
            info("Set to default input device");
            notifier.showToast("Set to default device!", "info");
        } else if (selectedDevice < 0) {
            warn(`User explicitly deactived mic.`);
            notifier.showToast("Deactivated microphones!", "warn");
        } else {
            info("Set to custom input device");
            notifier.showToast("Set new audio device!", "success");
        }
    }

    $effect(() => {
        getAudioInputs();
    });
</script>

<div
    class={`dropdown dropdown-${direction} z-10 tooltip tooltip-bottom ${className}`}
    data-tip=""
>
    <div class="tooltip-content">
        <div>
            Microphone permission is
            <span class={`text-${statusColor}`}>{statusValue}</span>
            <br />
            <span class="text-secondary">
                {selectedDevice >= 0
                    ? "Change the microphone"
                    : "Choose a microphone"}
            </span>
        </div>
    </div>
    <div
        tabindex="0"
        role="button"
        class={`btn m-1 btn-outline btn-${statusColor} text-xs`}
    >
        {#if showStatusAsEmoji}
            <span>{statusIcon}</span>
        {:else}
            <Status color={statusColor} size="lg" class="mr-2" />
        {/if}
        {icon}{icon && showName ? ": " : ""}
        {showName && name ? name : ""}
        <span class="text-accent">
            <!-- {configStore.theme.value}  -->
            {#if inputDevices.length > 0 && selectedDevice >= 0 && selectedDevice < inputDevices.length}
                {inputDevices[selectedDevice]}
            {:else if selectedDevice < 0}
                🚫
            {:else}
                Cannot get devices
            {/if}
            &#9660;
        </span>
    </div>
    <ul
        id="audio-input-list"
        class={`dropdown-content bg-base-300 rounded-box z-1 shadow-2xl ${listClass}`}
    >
        <li>
            <input
                tabindex="0"
                type="radio"
                name="audio-input-dropdown"
                class="btn btn-sm btn-block btn-ghost hover:btn-soft justify-start checked:border-error checked:border-2 text-center"
                aria-label="No Device"
                value={-1}
                checked={selectedDevice < 0}
                bind:group={selectedDevice}
                {onclick}
            />
        </li>
        {#each inputDevices as devices, idx}
            <li>
                <input
                    tabindex="0"
                    type="radio"
                    name="audio-input-dropdown"
                    class="btn btn-sm btn-block btn-ghost hover:btn-soft justify-start checked:border-success checked:border-2 text-center"
                    aria-label={devices}
                    value={idx}
                    checked={idx === selectedDevice}
                    bind:group={selectedDevice}
                    {onclick}
                />
            </li>
        {/each}
    </ul>
</div>
