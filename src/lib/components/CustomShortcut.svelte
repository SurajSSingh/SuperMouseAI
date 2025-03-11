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
    import type { NotificationSystem } from "$lib/notificationSystem.svelte";
    import { SvelteSet } from "svelte/reactivity";
    import type { Snippet } from "svelte";
    import { configStore } from "$lib/store.svelte";
    import { events } from "$lib/bindings";

    interface CustomShortcutProps {
        notifier?: NotificationSystem;
        onToggleShortcutEvent: ShortcutHandler;
        class?: string;
        description?: Snippet;
    }

    let {
        notifier,
        onToggleShortcutEvent,
        class: className = "",
        description,
    }: CustomShortcutProps = $props();

    type ShowShortcutFn = <T>(shortcut: string, returns: T) => T;

    const showShortcutFindingError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not check if ${shortcut} is registered (check for spelling issues)`,
            "Shortcut Finding Error",
        );
        return returns;
    };
    const showShortcutRegistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not register the shortcut: ${shortcut} (may have spelling issues or already be registered)`,
            "Shortcut Registration Error",
        );
        return returns;
    };
    const showShortcutUnregistrationError: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showError(
            `Could not unregister the shortcut: ${shortcut} (may have spelling issues or already be unregistered)`,
            "Shortcut Unregister Error",
        );
        return returns;
    };
    const showShortcutRegistrationSuccess: ShowShortcutFn = (
        shortcut: string,
        returns,
    ) => {
        notifier?.showSuccess(`Shortcut ${shortcut} has been registered`);
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
        configStore.shortcut = formatShortcutWith(keycode);
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
        configStore.shortcut = formatShortcutWith(
            mouseNumberToText(event.button),
        );
    }

    function toggleListen() {
        // Set previous shortcut before starting to listen
        if (!isListening) {
            previousShortcut = configStore.shortcut;
        }
        isListening = !isListening;
        // Register after finish listening
        if (!isListening) {
            setupShortcut(previousShortcut !== configStore.shortcut);
        }
    }

    async function setupShortcut(unregisterOld: boolean = false) {
        if (
            unregisterOld &&
            tauriRegistered &&
            (await isRegistered(previousShortcut).catch((e) =>
                showShortcutFindingError(previousShortcut, false),
            ))
        ) {
            await unregister(previousShortcut).catch((e) =>
                showShortcutUnregistrationError(previousShortcut, null),
            );
        }
        // For mouse click, don't register with Tauri's shortcut system
        if (configStore.shortcut.includes("Click")) {
            tauriRegistered = false;
        } else {
            // Check before registering (prevent re-registration error)
            try {
                const isShortcutReg = await isRegistered(configStore.shortcut);
                if (!isShortcutReg)
                    await register(
                        configStore.shortcut,
                        onToggleShortcutEvent,
                    ).then(
                        (_success) => {
                            tauriRegistered = true;
                        },
                        (_failure) =>
                            showShortcutRegistrationError(
                                configStore.shortcut,
                                null,
                            ),
                    );
            } catch (error) {
                showShortcutFindingError(configStore.shortcut, false);
                return;
            }
        }
        showShortcutRegistrationSuccess(configStore.shortcut, null);
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
            clickEventUnlistener = await events.mouseClickEvent.listen(
                (response) => {
                    if (isListening || tauriRegistered) return;
                    const payload = response.payload as string;
                    if (
                        checkAllModPressed() &&
                        configStore.shortcut.includes(payload)
                    ) {
                        onToggleShortcutEvent({
                            shortcut: "MouseClick",
                            id: 0,
                            state: "Pressed",
                        });
                    }
                },
            );
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
            await configStore.waitForStoreLoaded();
            const storedModKeys = configStore.modifierKeys;
            modAlt = storedModKeys.hasAlt;
            modCtrl = storedModKeys.hasControl;
            modShift = storedModKeys.hasShift;
            modSuper = storedModKeys.hasSuper;
            setupShortcut(previousShortcut === "");
        };
        asyncSetup();

        return () => {
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
                        setupShortcut(true);
                    } else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
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
                        setupShortcut(true);
                    } else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
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
                        setupShortcut(true);
                    } else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
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
                        setupShortcut(true);
                    } else {
                        notifier?.showInfo(
                            "Must have at least one modifer key!",
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
