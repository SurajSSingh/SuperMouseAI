<script lang="ts">
    import {
        isRegistered,
        register,
        unregister,
        unregisterAll,
        type ShortcutHandler,
    } from "@tauri-apps/plugin-global-shortcut";
    import Button from "./ui/button/button.svelte";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import {
        notifier,
        type NotificationSystem,
    } from "$lib/notificationSystem.svelte";
    import { SvelteSet } from "svelte/reactivity";
    import type { Snippet } from "svelte";
    import { configStore } from "$lib/store.svelte";
    import { events } from "$lib/bindings";
    import { debug, error, info, trace } from "@tauri-apps/plugin-log";

    interface CustomShortcutProps {
        onToggleShortcutEvent: ShortcutHandler;
        class?: string;
        description?: Snippet;
    }

    let {
        onToggleShortcutEvent,
        class: className = "",
        description,
    }: CustomShortcutProps = $props();

    type ShowShortcutFn = <T>(shortcut: string, returns: T) => T;

    const showShortcutFindingError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier.showToast(
            `Could not check if ${shortcut} is registered (check for spelling issues)`,
            "error",
            { subtitle: "Shortcut Finding Error" },
        );
        return returns;
    };
    const showShortcutRegistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier.showToast(
            `Could not register the shortcut: ${shortcut} (may have spelling issues or already be registered)`,
            "error",
            { subtitle: "Shortcut Registration Error" },
        );
        return returns;
    };
    const showShortcutUnregistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier.showToast(
            `Could not unregister the shortcut: ${shortcut} (may have spelling issues or already be unregistered)`,
            "error",
            { subtitle: "Shortcut Unregister Error" },
        );
        return returns;
    };
    const showShortcutRegistrationSuccess: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier.showToast(
            `Shortcut ${shortcut} has been registered`,
            "success",
        );
        return returns;
    };

    // https://github.com/tauri-apps/global-hotkey/blob/c9913a97667b3e44cb000de384cd8937d5a0050a/src/hotkey.rs#L199C20-L218C22
    let modAlt: boolean = $state(true);
    let modCtrl: boolean = $state(false);
    let modSuper: boolean = $state(false);
    let modShift: boolean = $state(true);

    let modCurrentSet: SvelteSet<string> = new SvelteSet();
    let isListening: boolean = $state(false);
    let tauriRegistered: boolean = $state(false);
    const LISTENER_BUTTON_ID = "shortcut-listener";
    let previousShortcut = $state("");

    const numberOfModKyes = $derived(
        +modCtrl + +modShift + +modAlt + +modSuper,
    );

    // Looking at https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    // for code values
    const EXCLUDED_MAIN_KEYS: string[] = [
        "Control",
        "Shift",
        "Alt",
        "Meta",
        "ControlLeft",
        "ShiftLeft",
        "AltLeft",
        "MetaLeft",
        "MetaRight",
        "ShiftRight",
        "AltRight",
    ];

    function onKeyDown(event: KeyboardEvent) {
        if (!isListening) return;
        modAlt = event.altKey;
        modCtrl = event.ctrlKey;
        modShift = event.shiftKey;
        modSuper = event.metaKey;
        const keycode = event.code;
        if (!keycode || EXCLUDED_MAIN_KEYS.includes(keycode)) return;
        configStore.shortcut.value = formatShortcutWith(keycode);
    }

    function mouseNumberToText(button: number): string {
        if (button === 0) {
            return "LeftClick";
        }
        if (button === 1) {
            return "MiddleClick";
        }
        if (button === 2) {
            return "RightClick";
        }
        return `MouseButton${button + 1}Click`;
    }

    function formatShortcutWith(mainKey: string) {
        return `${modCtrl ? "Control+" : ""}${modShift ? "Shift+" : ""}${modAlt ? "Alt+" : ""}${modSuper ? "Super+" : ""}${mainKey}`;
    }

    function onMouseDown(event: MouseEvent) {
        if (
            !isListening ||
            // Don't change if clicks to stop listening
            // @ts-ignore .id on target exists
            event.target?.id === LISTENER_BUTTON_ID ||
            // Don't allow unmodified clicks
            !(event.altKey || event.ctrlKey || event.shiftKey || event.metaKey)
        )
            return;
        modAlt = event.altKey;
        modCtrl = event.ctrlKey;
        modShift = event.shiftKey;
        modSuper = event.metaKey;
        configStore.shortcut.value = formatShortcutWith(
            mouseNumberToText(event.button),
        );
    }

    function toggleListen() {
        info(`Toggle listen for new shortcut`);
        // Set previous shortcut before starting to listen
        if (!isListening) {
            trace(`Set previous shortcut`);
            previousShortcut = configStore.shortcut.value;
        }
        isListening = !isListening;
        debug(
            `${isListening ? "Start listening for new shortcut" : "Stop listening for new shortcut"}`,
        );
        // Register after finish listening
        if (!isListening) {
            trace(`Setup new shortcut`);
            setupShortcut(previousShortcut !== configStore.shortcut.value);
        }
    }

    async function setupShortcut(
        unregisterOld = false,
        modifierUpdate = false,
    ) {
        info(`Setting up new shortcut`);
        debug(
            `Unregistering old = ${unregisterOld}; same main key, new modifers = ${modifierUpdate}`,
        );
        if (modifierUpdate) {
            trace(`Updating shortcut with new modifier`);
            configStore.shortcut.value = formatShortcutWith(
                configStore.mainKey,
            );
        }
        if (
            unregisterOld &&
            tauriRegistered &&
            (await isRegistered(previousShortcut).catch((e) =>
                showShortcutFindingError(previousShortcut, false),
            ))
        ) {
            debug(`Unregistering old shortcut: ${previousShortcut}`);
            await unregister(previousShortcut).catch((e) =>
                showShortcutUnregistrationError(previousShortcut, null),
            );
            trace(`Unregistered old shortcut`);
        }
        // For mouse click, don't register with Tauri's shortcut system
        if (configStore.shortcut.value.includes("Click")) {
            trace(`Register modified mouse click shortcut`);
            tauriRegistered = false;
        } else {
            // Check before registering (prevent re-registration error)
            try {
                debug(
                    `Attempting to register new shortcut: ${configStore.shortcut.value}`,
                );
                const isShortcutReg = await isRegistered(
                    configStore.shortcut.value,
                );
                trace(`Shortcut already registered = ${isShortcutReg}`);
                if (!isShortcutReg)
                    await register(
                        configStore.shortcut.value,
                        onToggleShortcutEvent,
                    ).then(
                        (_success) => {
                            tauriRegistered = true;
                        },
                        (_failure) => {
                            error(
                                `Could not register shortcut: ${configStore.shortcut.value}`,
                            );
                            return showShortcutRegistrationError(
                                configStore.shortcut.value,
                                null,
                            );
                        },
                    );
            } catch (err) {
                error(`Shortcut registration error: ${err}`);
                showShortcutFindingError(configStore.shortcut.value, false);
                return;
            }
        }
        info(`Registed new shortcut: ${configStore.shortcut.value}`);
        showShortcutRegistrationSuccess(configStore.shortcut.value, null);
    }

    let clickEventUnlistener: UnlistenFn | null = null;
    let modKeyEventUnlistener: UnlistenFn | null = null;

    let hasShortcutError = $state(false);

    function checkAllModPressed(): boolean {
        return (
            modCurrentSet.has("Super") === modSuper &&
            modCurrentSet.has("Shift") === modShift &&
            modCurrentSet.has("Control") === modCtrl &&
            modCurrentSet.has("Alt") === modAlt
        );
    }

    $effect(() => {
        addEventListener("keydown", onKeyDown);
        addEventListener("mousedown", onMouseDown);
        const asyncSetup = async () => {
            debug(`Setup Shortcut event listeners`);
            clickEventUnlistener = await events.mouseClickEvent.listen(
                (response) => {
                    if (isListening || tauriRegistered) return;
                    const payload = response.payload as string;
                    if (
                        checkAllModPressed() &&
                        configStore.shortcut.value.includes(payload)
                    ) {
                        onToggleShortcutEvent({
                            shortcut: "MouseClick",
                            id: 0,
                            state: "Pressed",
                        });
                    }
                },
            );
            trace(`Mouse click event listener set up`);
            modKeyEventUnlistener = await events.modKeyEvent.listen(
                (response) => {
                    if (isListening || tauriRegistered) return;
                    const key = response.payload.key
                        .replace(/(?:L(?:eft)?|R(?:ight))?(.*)/g, "$1")
                        .replace("Option", "Alt")
                        .replace("Command", "Super");
                    if (response.payload.is_pressed) {
                        modCurrentSet.add(key);
                    } else {
                        modCurrentSet.delete(key);
                    }
                },
            );
            trace(`Mod key event listener set up`);
            await configStore.waitForStoreLoaded();
            trace(`configStore is fully loaded`);
            const storedModKeys = configStore.modifierKeys;
            modAlt = storedModKeys.hasAlt;
            modCtrl = storedModKeys.hasControl;
            modShift = storedModKeys.hasShift;
            modSuper = storedModKeys.hasSuper;
            trace(`Loaded modifier key from config`);
            setupShortcut(previousShortcut === "");
        };
        debug(`Start Custom Shortcut component setup`);
        asyncSetup();

        return () => {
            debug(`Start Custom Shortcut component clean up`);
            removeEventListener("keydown", onKeyDown);
            removeEventListener("mousedown", onMouseDown);
            try {
                clickEventUnlistener?.();
                modKeyEventUnlistener?.();
                unregisterAll();
            } catch (error) {
                console.error(error);
            }
        };
    });
</script>

{#snippet keyboardItem(
    name: string,
    active?: boolean,
    enabled?: boolean,
    onclick?: () => void,
    usePlus: boolean = false,
)}
    {#if enabled ?? true}
        <button {onclick}>
            <kbd
                class={`kbd text-xs sm:text-sm ${active ? "outline-2 outline-primary" : ""} ${numberOfModKyes > 1 && onclick ? "hover:bg-error" : ""}`}
                >{name}</kbd
            >
        </button>
        {#if usePlus}
            <span class="px-1 m-0">+</span>
        {/if}
    {/if}
{/snippet}

<section class="flex justify-center place-center">
    <fieldset
        class={`fieldset bg-base-200 border border-base-300 p-4 rounded-box ${hasShortcutError ? "border-error" : "border-success"} ${className}`}
    >
        <legend class="fieldset-legend">Recording Shortcut</legend>
        <p class="flex justify-center align-text-top">
            {@render keyboardItem(
                "Ctrl",
                false,
                modCtrl,
                () => {
                    if (numberOfModKyes > 1) {
                        modCtrl = false;
                        setupShortcut(true, true);
                    } else {
                        notifier.showToast(
                            "Must have at least one modifer key!",
                            "warn",
                            { sound: "alert" },
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Shift",
                false,
                modShift,
                () => {
                    if (numberOfModKyes > 1) {
                        modShift = false;
                        setupShortcut(true, true);
                    } else {
                        notifier.showToast(
                            "Must have at least one modifer key!",
                            "warn",
                            { sound: "alert" },
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Alt",
                false,
                modAlt,
                () => {
                    if (numberOfModKyes > 1) {
                        modAlt = false;
                        setupShortcut(true, true);
                    } else {
                        notifier.showToast(
                            "Must have at least one modifer key!",
                            "warn",
                            { sound: "alert" },
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                "Super",
                false,
                modSuper,
                () => {
                    if (numberOfModKyes > 1) {
                        modSuper = false;
                        setupShortcut(true, true);
                    } else {
                        notifier.showToast(
                            "Must have at least one modifer key!",
                            "warn",
                            { sound: "alert" },
                        );
                    }
                },
                true,
            )}
            {@render keyboardItem(
                configStore.mainKey.replace(/([a-z0-9])([A-Z])/g, "$1 $2"),
                false,
                true,
            )}
        </p>
        <Button
            id={LISTENER_BUTTON_ID}
            color={isListening ? "primary" : "info"}
            class="h-6"
            onclick={toggleListen}
        >
            <span class="text-xs"> Listen for new shortcut </span>
        </Button>
        <div class="fieldset-label">
            {@render description?.()}
        </div>
    </fieldset>
</section>
